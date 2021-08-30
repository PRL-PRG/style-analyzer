# Test report for javascript / file:///tmp/top-repos-quality-repos-u09lph72/schoolwork.git HEAD 1afb6b507d106a90c2493e4b4bb632a160dfe3e5

### Classification report

PPCR: 0.875

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.981| 0.929| 0.985| 0.958| 16663| 17587| 0.947 |
| `␣` | 0.958| 0.988| 0.949| 0.973| 0.953| 8953| 9325| 0.960 |
| `⏎␣⁺␣⁺` | 0.974| 0.950| 0.771| 0.962| 0.861| 862| 1062| 0.812 |
| `⏎` | 0.847| 0.916| 0.447| 0.880| 0.585| 763| 1564| 0.488 |
| `'` | 0.959| 0.965| 0.504| 0.962| 0.661| 746| 1428| 0.522 |
| `⏎␣⁻␣⁻` | 0.996| 0.892| 0.500| 0.941| 0.666| 582| 1037| 0.561 |
| `⏎⏎` | 0.973| 0.841| 0.430| 0.902| 0.597| 509| 995| 0.512 |
| `"` | 0.770| 0.681| 0.216| 0.723| 0.338| 113| 356| 0.317 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 53| 54| 0.981 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.972| 0.973| 0.852| 0.972| 0.896| 29244| 33408| 0.875 |
| `macro avg` | 0.533| 0.515| 0.339| 0.523| 0.401| 29244| 33408| 0.875 |
| `micro avg` | 0.973| 0.973| 0.852| 0.973| 0.908| 29244| 33408| 0.875 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎⇥⁻| ⏎⏎⇥⁺| ⏎⏎⏎| ⏎⇥⁻⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|924 |16339 |301 |0 |0 |0 |0 |21 |2 |0 |0 |0 |
|372 |90 |8848 |7 |0 |7 |0 |1 |0 |0 |0 |0 |
|801 |17 |42 |699 |0 |5 |0 |0 |0 |0 |0 |0 |
|682 |2 |1 |0 |720 |0 |23 |0 |0 |0 |0 |0 |
|486 |4 |14 |63 |0 |428 |0 |0 |0 |0 |0 |0 |
|243 |0 |5 |0 |31 |0 |77 |0 |0 |0 |0 |0 |
|200 |19 |21 |0 |0 |0 |0 |819 |0 |3 |0 |0 |
|455 |39 |3 |3 |0 |0 |0 |0 |519 |0 |18 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |53 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.7230046948356806, "precision": 0.77, "recall": 0.6814159292035398, "support": 113}, "\u0027": {"f1-score": 0.9619238476953907, "precision": 0.9587217043941412, "recall": 0.9651474530831099, "support": 746}, "macro avg": {"f1-score": 0.5234437563620741, "precision": 0.5333186033086162, "recall": 0.5153031996759936, "support": 29244}, "micro avg": {"f1-score": 0.9728149363972097, "precision": 0.9728149363972097, "recall": 0.9728149363972097, "support": 29244}, "weighted avg": {"f1-score": 0.9722369503126712, "precision": 0.9722080738217665, "recall": 0.9728149363972097, "support": 29244}, "\u2205": {"f1-score": 0.9850782262683507, "precision": 0.9896426408237432, "recall": 0.980555722258897, "support": 16663}, "\u23ce": {"f1-score": 0.880352644836272, "precision": 0.8472727272727273, "recall": 0.9161205766710354, "support": 763}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.9020021074815596, "precision": 0.9727272727272728, "recall": 0.8408644400785854, "support": 509}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 53}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9618320610687023, "precision": 0.9738406658739596, "recall": 0.9501160092807425, "support": 862}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9410698096101542, "precision": 0.9961612284069098, "recall": 0.8917525773195877, "support": 582}, "\u2423": {"f1-score": 0.9729491972729272, "precision": 0.9580942068218733, "recall": 0.9882720875684128, "support": 8953}},
  "cl_report_full": {"\"": {"f1-score": 0.33771929824561403, "precision": 0.77, "recall": 0.21629213483146068, "support": 356}, "\u0027": {"f1-score": 0.6608536025699862, "precision": 0.9587217043941412, "recall": 0.5042016806722689, "support": 1428}, "macro avg": {"f1-score": 0.40136338100314545, "precision": 0.5333186033086162, "recall": 0.3390806996891609, "support": 33408}, "micro avg": {"f1-score": 0.9081593564451255, "precision": 0.9728149363972097, "recall": 0.8515625, "support": 33408}, "weighted avg": {"f1-score": 0.8957040710139499, "precision": 0.9681059734042824, "recall": 0.8515625, "support": 33408}, "\u2205": {"f1-score": 0.9583834354928586, "precision": 0.9896426408237432, "recall": 0.929038494342412, "support": 17587}, "\u23ce": {"f1-score": 0.5851820845542067, "precision": 0.8472727272727273, "recall": 0.4469309462915601, "support": 1564}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.5965156794425087, "precision": 0.9727272727272728, "recall": 0.4301507537688442, "support": 995}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.860746190225959, "precision": 0.9738406658739596, "recall": 0.7711864406779662, "support": 1062}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6662387676508345, "precision": 0.9961612284069098, "recall": 0.5004821600771456, "support": 1037}, "\u2423": {"f1-score": 0.9534482758620689, "precision": 0.9580942068218733, "recall": 0.9488471849865951, "support": 9325}},
  "ppcr": 0.8753591954022989
}
```
</details>