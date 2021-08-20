# Train report for javascript / file:///tmp/top-repos-quality-repos-td5wtewu/event-admin.git HEAD 774b6aa49cd816b004202f065c8e14ecaf707898

### Classification report

PPCR: 0.230

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.196| 1.000| 0.327| 504| 2577| 0.196 |
| `"` | 1.000| 1.000| 0.822| 1.000| 0.903| 449| 546| 0.822 |
| `␣` | 0.943| 1.000| 0.158| 0.970| 0.271| 164| 1036| 0.158 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 216| 0.046 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 332| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 195| 0.000 |
| `macro avg` | 0.490| 0.500| 0.196| 0.495| 0.250| 1127| 4902| 0.230 |
| `weighted avg` | 0.983| 0.991| 0.228| 0.987| 0.330| 1127| 4902| 0.230 |
| `micro avg` | 0.991| 0.991| 0.228| 0.991| 0.371| 1127| 4902| 0.230 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|2073 |504 |0 |0 |0 |0 |0 |
|872 |0 |164 |0 |0 |0 |0 |
|97 |0 |0 |449 |0 |0 |0 |
|332 |0 |0 |0 |0 |0 |0 |
|206 |0 |10 |0 |0 |0 |0 |
|195 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| live-site/src/components/Leaderboard.js | 3 |
| live-site/src/components/Announcements.js | 2 |
| live-site/src/components/Schedule.js | 2 |
| admin/src/components/SendAnnouncement.js | 1 |
| live-site/src/components/Team.js | 1 |
| admin/src/pages/LogIn.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 449}, "macro avg": {"f1-score": 0.495069033530572, "precision": 0.4904214559386973, "recall": 0.5, "support": 1127}, "micro avg": {"f1-score": 0.9911268855368234, "precision": 0.9911268855368234, "recall": 0.9911268855368234, "support": 1127}, "weighted avg": {"f1-score": 0.9868215873949271, "precision": 0.982763720180726, "recall": 0.9911268855368234, "support": 1127}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 504}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9704142011834319, "precision": 0.9425287356321839, "recall": 1.0, "support": 164}},
  "cl_report_full": {"\"": {"f1-score": 0.9025125628140703, "precision": 1.0, "recall": 0.8223443223443223, "support": 546}, "macro avg": {"f1-score": 0.25012557456017565, "precision": 0.4904214559386973, "recall": 0.19603695535011015, "support": 4902}, "micro avg": {"f1-score": 0.3705423785038978, "precision": 0.9911268855368234, "recall": 0.22786617707058343, "support": 4902}, "weighted avg": {"f1-score": 0.3298068133291203, "precision": 0.8362831028386255, "recall": 0.22786617707058343, "support": 4902}, "\u2205": {"f1-score": 0.32716650438169426, "precision": 1.0, "recall": 0.19557625145518046, "support": 2577}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 332}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 216}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 195}, "\u2423": {"f1-score": 0.27107438016528923, "precision": 0.9425287356321839, "recall": 0.1583011583011583, "support": 1036}},
  "ppcr": 0.229906160750714
}
```
</details>
