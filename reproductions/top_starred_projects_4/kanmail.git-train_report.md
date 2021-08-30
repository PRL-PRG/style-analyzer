# Train report for javascript / file:///tmp/top-repos-quality-repos-o1z9cpbc/kanmail.git HEAD 98699a14fa32aa1fd6d7384328ca30da6aae7a01

### Classification report

PPCR: 0.518

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.982| 0.997| 0.751| 0.989| 0.851| 5730| 7607| 0.753 |
| `␣` | 0.967| 0.881| 0.173| 0.922| 0.294| 675| 3434| 0.197 |
| `'` | 1.000| 1.000| 0.816| 1.000| 0.898| 504| 618| 0.816 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.997| 0.945| 0.569| 0.971| 0.725| 348| 578| 0.602 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 694| 0.004 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 590| 0.002 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 498| 0.000 |
| `macro avg` | 0.564| 0.546| 0.330| 0.555| 0.395| 7261| 14019| 0.518 |
| `weighted avg` | 0.982| 0.983| 0.509| 0.982| 0.603| 7261| 14019| 0.518 |
| `micro avg` | 0.983| 0.983| 0.509| 0.983| 0.671| 7261| 14019| 0.518 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1877 |5710 |20 |0 |0 |0 |0 |0 |
|2759 |79 |595 |0 |0 |0 |1 |0 |
|691 |3 |0 |0 |0 |0 |0 |0 |
|114 |0 |0 |0 |504 |0 |0 |0 |
|589 |1 |0 |0 |0 |0 |0 |0 |
|230 |19 |0 |0 |0 |0 |329 |0 |
|498 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| kanmail/client/util/threads.js | 15 |
| kanmail/client/stores/emails/main.js | 11 |
| kanmail/client/stores/request.js | 10 |
| kanmail/client/threading.js | 10 |
| kanmail/client/stores/emails/base.js | 10 |
| kanmail/client/stores/thread.js | 9 |
| kanmail/client/util/requests.js | 9 |
| kanmail/client/stores/columns.js | 8 |
| kanmail/client/stores/settings.js | 7 |
| kanmail/client/stores/emails/search.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 504}, "macro avg": {"f1-score": 0.5546302861775414, "precision": 0.5638427821430189, "recall": 0.5461990541336994, "support": 7261}, "micro avg": {"f1-score": 0.9830601845475829, "precision": 0.9830601845475829, "recall": 0.9830601845475829, "support": 7261}, "weighted avg": {"f1-score": 0.9824875782460222, "precision": 0.9824312527982393, "recall": 0.9830601845475829, "support": 7261}, "\u2205": {"f1-score": 0.9894299081614971, "precision": 0.9824501032346868, "recall": 0.9965095986038395, "support": 5730}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9705014749262536, "precision": 0.996969696969697, "recall": 0.9454022988505747, "support": 348}, "\u2423": {"f1-score": 0.9224806201550388, "precision": 0.967479674796748, "recall": 0.8814814814814815, "support": 675}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8983957219251336, "precision": 1.0, "recall": 0.8155339805825242, "support": 618}, "macro avg": {"f1-score": 0.3954281674878711, "precision": 0.5638427821430189, "recall": 0.3298042692051658, "support": 14019}, "micro avg": {"f1-score": 0.6708646616541353, "precision": 0.9830601845475829, "recall": 0.5091661316784364, "support": 14019}, "weighted avg": {"f1-score": 0.6032613296627727, "precision": 0.8552729597979014, "recall": 0.5091661316784364, "support": 14019}, "\u2205": {"f1-score": 0.8510321186377524, "precision": 0.9824501032346868, "recall": 0.7506244248718286, "support": 7607}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 694}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 498}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 590}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.724669603524229, "precision": 0.996969696969697, "recall": 0.5692041522491349, "support": 578}, "\u2423": {"f1-score": 0.2938997283279822, "precision": 0.967479674796748, "recall": 0.17326732673267325, "support": 3434}},
  "ppcr": 0.5179399386546829
}
```
</details>
