# Train report for javascript / file:///tmp/top-repos-quality-repos-dr3uypbr/covid19india-react.git HEAD b9add2bedc564b94aadacc7bb164372115274eb6

### Classification report

PPCR: 0.802

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.944| 0.998| 0.938| 0.970| 0.941| 21558| 22934| 0.940 |
| `␣` | 0.978| 0.957| 0.650| 0.968| 0.781| 6398| 9420| 0.679 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 2926| 2926| 1.000 |
| `⏎␣⁺␣⁺` | 0.939| 0.384| 0.247| 0.545| 0.391| 928| 1443| 0.643 |
| `⏎␣⁻␣⁻` | 0.996| 0.433| 0.216| 0.603| 0.355| 615| 1232| 0.499 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 520| 520| 1.000 |
| `⏎` | 0.994| 0.712| 0.076| 0.830| 0.141| 243| 2286| 0.106 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 82| 141| 0.582 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 78| 0.410 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 573| 0.023 |
| `micro avg` | 0.957| 0.957| 0.767| 0.957| 0.851| 33315| 41553| 0.802 |
| `weighted avg` | 0.954| 0.957| 0.767| 0.949| 0.811| 33315| 41553| 0.802 |
| `macro avg` | 0.685| 0.548| 0.413| 0.592| 0.461| 33315| 41553| 0.802 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1376 |21507 |33 |0 |0 |18 |0 |0 |0 |0 |0 |
|3022 |271 |6126 |0 |1 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |2926 |0 |0 |0 |0 |0 |0 |0 |
|2043 |47 |23 |0 |173 |0 |0 |0 |0 |0 |0 |
|515 |507 |65 |0 |0 |356 |0 |0 |0 |0 |0 |
|617 |334 |15 |0 |0 |0 |266 |0 |0 |0 |0 |
|560 |13 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |520 |0 |0 |
|59 |81 |0 |0 |0 |0 |1 |0 |0 |0 |0 |
|46 |27 |0 |0 |0 |5 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/Timeseries.js | 151 |
| src/components/MapVisualizer.js | 127 |
| src/components/MapLegend.js | 78 |
| src/components/TimeseriesExplorer.js | 74 |
| src/components/Table.js | 73 |
| src/components/Search.js | 71 |
| src/components/Row.js | 67 |
| src/components/State.js | 66 |
| src/serviceWorker.js | 58 |
| src/components/TimeseriesBrush.js | 49 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 520}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2926}, "macro avg": {"f1-score": 0.5915427658310857, "precision": 0.685193085952966, "recall": 0.5483196174730411, "support": 33315}, "micro avg": {"f1-score": 0.9567462104157286, "precision": 0.9567462104157286, "recall": 0.9567462104157286, "support": 33315}, "weighted avg": {"f1-score": 0.9493281453106557, "precision": 0.953866595217081, "recall": 0.9567462104157286, "support": 33315}, "\u2205": {"f1-score": 0.9699853422031797, "precision": 0.9438276210119805, "recall": 0.9976342888950738, "support": 21558}, "\u23ce": {"f1-score": 0.829736211031175, "precision": 0.9942528735632183, "recall": 0.7119341563786008, "support": 243}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5447589900535578, "precision": 0.9393139841688655, "recall": 0.38362068965517243, "support": 928}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6031746031746033, "precision": 0.9962546816479401, "recall": 0.432520325203252, "support": 615}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 82}, "\u2423": {"f1-score": 0.9677725118483413, "precision": 0.9782816991376557, "recall": 0.957486714598312, "support": 6398}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 520}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2926}, "macro avg": {"f1-score": 0.46084040079319155, "precision": 0.685193085952966, "recall": 0.4126391820770481, "support": 41553}, "micro avg": {"f1-score": 0.8514719239194316, "precision": 0.9567462104157286, "recall": 0.7670685630399731, "support": 41553}, "weighted avg": {"f1-score": 0.8111194948572273, "precision": 0.9424791033638228, "recall": 0.7670685630399731, "support": 41553}, "\u2205": {"f1-score": 0.9407930710176942, "precision": 0.9438276210119805, "recall": 0.9377779715705938, "support": 22934}, "\u23ce": {"f1-score": 0.14065040650406505, "precision": 0.9942528735632183, "recall": 0.07567804024496938, "support": 2286}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 573}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.3907793633369923, "precision": 0.9393139841688655, "recall": 0.2467082467082467, "support": 1443}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 78}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.3549032688458973, "precision": 0.9962546816479401, "recall": 0.2159090909090909, "support": 1232}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 141}, "\u2423": {"f1-score": 0.781277898227267, "precision": 0.9782816991376557, "recall": 0.6503184713375796, "support": 9420}},
  "ppcr": 0.8017471662695834
}
```
</details>
