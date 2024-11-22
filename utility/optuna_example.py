# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "optuna",
#     "scipy",
#     "tqdm",
# ]
# ///

import optuna
import concurrent.futures
import subprocess
import os
import numpy as np
from tqdm import tqdm

N_PARALLEL = 4
N_FILES = 100
IN_FOLDER = "./tools/in"
OUT_FOLDER = "./tools/out"

assert(os.path.isdir(IN_FOLDER))
assert(os.path.isdir(OUT_FOLDER))

def calc_score_each(seed: int, in_file: str, out_file: str, l_ratio: float, r_ratio: float):
    proc = subprocess.run(f"./target/release/ahc007 {l_ratio} {r_ratio} < {in_file} > {out_file} && ./vis {in_file} {out_file}", shell=True, capture_output=True)
    # stderrから取得したい場合: proc.stderr.deocde().split()
    return seed, int(proc.stdout.decode().split()[-1]) / 1e8

def params_from_trial(trial: optuna.trial.Trial):
    l_ratio = trial.suggest_float("l_ratio", 0.6, 1.4)
    r_ratio = trial.suggest_float("r_ratio", 2.6, 3.4)
    return (l_ratio, r_ratio)

def objective(trial: optuna.trial.Trial):
    params = params_from_trial(trial)
    seeds = np.random.permutation(N_FILES)
    results = []
    with concurrent.futures.ProcessPoolExecutor(max_workers=N_PARALLEL) as executor:
        futures = [executor.submit(
                calc_score_each,
                seed,
                f"{IN_FOLDER}/{seed:04}.txt",
                f"{OUT_FOLDER}/{seed:04}.txt",
                *params,
            ) for seed in seeds]
        pruned = False
        for future in tqdm(concurrent.futures.as_completed(futures), total=N_FILES):
            if future.cancelled():
                continue
            seed, score = future.result()
            results.append(score)
            trial.report(score, seed)
            if not pruned and trial.should_prune():
                pruned = True
                print("pruned")
                executor.shutdown(wait=True, cancel_futures=True)
                break
    result = sum(results) / len(results)
    print(f"result = {result}")
    return result
if __name__ == "__main__":
    study = optuna.create_study(
        pruner=optuna.pruners.WilcoxonPruner(p_threshold=0.1),
        direction="maximize",
        storage="sqlite:///example.db",
        study_name="tune_range2",
        load_if_exists=True,
    )
    study.optimize(objective, n_trials=10000)
