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
N_FILES = 50
IN_FOLDER = "./tools/in"
OUT_FOLDER = "./tools/out"

base_score = [
774373
,1421265
,685520
,1255857
,700604
,1118531
,849833
,1034383
,931977
,973925
,853363
,1234624
,1103404
,1034775
,1161092
,986949
,897837
,1361745
,1018649
,890175
,939647
,1303443
,942953
,1221082
,1009860
,827238
,1137985
,1498573
,1538186
,1133315
,896105
,1398712
,1174259
,1306184
,1169912
,1334182
,1405482
,1336786
,750584
,1076814
,1197764
,828636
,1303983
,735708
,963818
,802102
,1206140
,1328493
,822837
,779490
,1323331
,1185215
,1060256
,980997
,962626
,1017691
,1482106
,920651
,1251367
,1120673
,1071197
,1191248
,1009847
,1212807
,1406252
,1252990
,1214298
,1147727
,948199
,698248
,790695
,1004346
,866817
,932832
,1303269
,923390
,1048601
,954915
,830642
,1294780
,1459685
,1086241
,815964
,1195309
,1155366
,1062553
,1234261
,1090265
,790087
,935721
,1426062
,1349984
,748300
,1121892
,1199463
,1510005
,1539593
,1262163
,1035908
,887110
]

assert(os.path.isdir(IN_FOLDER))
assert(os.path.isdir(OUT_FOLDER))

def calc_score_each(seed: int, in_file: str, out_file: str, t0: float, t1: float):
    proc = subprocess.run(f"./run ./target/release/ahc040 {t0} {t1} < {in_file} > {out_file}", shell=True, capture_output=True)
    # stderrから取得したい場合: proc.stderr.deocde().split()
    return seed, float(proc.stderr.decode().split()[-1]) / float(base_score[seed])

def params_from_trial(trial: optuna.trial.Trial):
    t0 = trial.suggest_float("T0", 500.0, 4000.0)
    t1 = trial.suggest_float("T1", 10.0, 500.0)
    return (t0, t1)

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
        direction="minimize",
        storage="sqlite:///example.db",
        study_name="temp0",
        load_if_exists=True,
    )
    study.optimize(objective, n_trials=10000)
