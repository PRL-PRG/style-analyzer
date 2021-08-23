# Train report for javascript / file:///tmp/top-repos-quality-repos-949p9w9n/pslab-desktop.git HEAD 7e8a5b1f8c3982219a57ba2dd4ab8e16758ea5b3

### Classification report

PPCR: 0.674

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.974| 0.997| 0.757| 0.985| 0.852| 25720| 33891| 0.759 |
| `␣` | 0.984| 0.943| 0.597| 0.963| 0.743| 6851| 10824| 0.633 |
| `'` | 1.000| 1.000| 0.912| 1.000| 0.954| 3213| 3524| 0.912 |
| `⏎␣⁻␣⁻` | 0.979| 0.848| 0.537| 0.909| 0.693| 1267| 2003| 0.633 |
| `"` | 1.000| 1.000| 0.491| 1.000| 0.659| 715| 1456| 0.491 |
| `⏎` | 1.000| 0.930| 0.172| 0.964| 0.294| 644| 3482| 0.185 |
| `⏎⏎` | 0.953| 0.997| 0.490| 0.975| 0.647| 288| 586| 0.491 |
| `⏎␣⁺␣⁺` | 1.000| 0.564| 0.080| 0.721| 0.148| 280| 1983| 0.141 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 106| 0.094 |
| `weighted avg` | 0.979| 0.979| 0.660| 0.978| 0.766| 38988| 57855| 0.674 |
| `micro avg` | 0.979| 0.979| 0.660| 0.979| 0.788| 38988| 57855| 0.674 |
| `macro avg` | 0.877| 0.809| 0.448| 0.835| 0.554| 38988| 57855| 0.674 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| "| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8171 |25651 |62 |0 |0 |7 |0 |0 |0 |0 |
|3973 |376 |6459 |0 |0 |16 |0 |0 |0 |0 |
|311 |0 |0 |3213 |0 |0 |0 |0 |0 |0 |
|2838 |9 |22 |0 |599 |0 |0 |0 |14 |0 |
|736 |188 |4 |0 |0 |1075 |0 |0 |0 |0 |
|1703 |117 |5 |0 |0 |0 |158 |0 |0 |0 |
|741 |0 |0 |0 |0 |0 |0 |715 |0 |0 |
|298 |1 |0 |0 |0 |0 |0 |0 |287 |0 |
|96 |0 |10 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/Appshell/Appshell.js | 92 |
| src/App.js | 46 |
| src/screen/PowerSource/components/InstrumentCluster.js | 44 |
| src/serviceWorker.js | 33 |
| src/screen/AboutUs/AboutUs.js | 33 |
| src/screen/Oscilloscope/components/ChannelParameters.js | 31 |
| src/screen/WaveGenerator/components/DigitalController.js | 29 |
| src/screen/LogicAnalyzer/components/ChannelParameters.js | 25 |
| utils/preProcessor.js | 25 |
| src/screen/Oscilloscope/Oscilloscope.js | 24 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 715}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3213}, "macro avg": {"f1-score": 0.8352644436641998, "precision": 0.876734765371076, "recall": 0.808833109493302, "support": 38988}, "micro avg": {"f1-score": 0.9786857494613728, "precision": 0.9786857494613728, "recall": 0.9786857494613728, "support": 38988}, "weighted avg": {"f1-score": 0.9778876977390725, "precision": 0.9786561130068958, "recall": 0.9786857494613728, "support": 38988}, "\u2205": {"f1-score": 0.9854020206676656, "precision": 0.9737681269455623, "recall": 0.9973172628304822, "support": 25720}, "\u23ce": {"f1-score": 0.9637972646822205, "precision": 1.0, "recall": 0.9301242236024845, "support": 644}, "\u23ce\u23ce": {"f1-score": 0.9745331069609507, "precision": 0.9534883720930233, "recall": 0.9965277777777778, "support": 288}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7214611872146118, "precision": 1.0, "recall": 0.5642857142857143, "support": 280}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9090909090909091, "precision": 0.9790528233151184, "recall": 0.8484609313338595, "support": 1267}, "\u2423": {"f1-score": 0.9630955043614404, "precision": 0.9843035659859799, "recall": 0.9427820756094001, "support": 6851}},
  "cl_report_full": {"\"": {"f1-score": 0.6586826347305389, "precision": 1.0, "recall": 0.49107142857142855, "support": 1456}, "\u0027": {"f1-score": 0.9538370194448568, "precision": 1.0, "recall": 0.9117480136208853, "support": 3524}, "macro avg": {"f1-score": 0.5543174476471703, "precision": 0.876734765371076, "recall": 0.44828637932675997, "support": 57855}, "micro avg": {"f1-score": 0.7880177194015057, "precision": 0.9786857494613728, "recall": 0.6595281306715064, "support": 57855}, "weighted avg": {"f1-score": 0.7659034732090961, "precision": 0.9786684708269119, "recall": 0.6595281306715064, "support": 57855}, "\u2205": {"f1-score": 0.8517257981505156, "precision": 0.9737681269455623, "recall": 0.7568676049688708, "support": 33891}, "\u23ce": {"f1-score": 0.2935555011026709, "precision": 1.0, "recall": 0.172027570361861, "support": 3482}, "\u23ce\u23ce": {"f1-score": 0.6471251409244646, "precision": 0.9534883720930233, "recall": 0.48976109215017066, "support": 586}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.14759458197104158, "precision": 1.0, "recall": 0.07967725668179526, "support": 1983}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 106}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6933247339567882, "precision": 0.9790528233151184, "recall": 0.5366949575636545, "support": 2003}, "\u2423": {"f1-score": 0.7430116185436558, "precision": 0.9843035659859799, "recall": 0.5967294900221729, "support": 10824}},
  "ppcr": 0.6738916256157635
}
```
</details>
