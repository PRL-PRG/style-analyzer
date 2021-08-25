# Train report for javascript / file:///tmp/top-repos-quality-repos-_suuzb1g/spotify-playlist-import.git HEAD 9b713aade4cbb076b5456fc475e37a0f38fb5523

### Classification report

PPCR: 0.249

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.997| 1.000| 0.350| 0.998| 0.519| 328| 936| 0.350 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 387| 0.003 |
| `weighted avg` | 0.994| 0.997| 0.248| 0.995| 0.367| 329| 1323| 0.249 |
| `micro avg` | 0.997| 0.997| 0.248| 0.997| 0.397| 329| 1323| 0.249 |
| `macro avg` | 0.498| 0.500| 0.175| 0.499| 0.259| 329| 1323| 0.249 |

### Confusion matrix

|refusal|  ∅| ␣| 
|:---|:---|:---|
|0 |0 |0 |
|608 |328 |0 |
|386 |1 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| server.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.4992389649923897, "precision": 0.49848024316109424, "recall": 0.5, "support": 329}, "micro avg": {"f1-score": 0.9969604863221885, "precision": 0.9969604863221885, "recall": 0.9969604863221885, "support": 329}, "weighted avg": {"f1-score": 0.9954430426595977, "precision": 0.9939302112877746, "recall": 0.9969604863221885, "support": 329}, "\u2205": {"f1-score": 0.9984779299847794, "precision": 0.9969604863221885, "recall": 1.0, "support": 328}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}},
  "cl_report_full": {"macro avg": {"f1-score": 0.2592885375494071, "precision": 0.49848024316109424, "recall": 0.1752136752136752, "support": 1323}, "micro avg": {"f1-score": 0.39709443099273606, "precision": 0.9969604863221885, "recall": 0.24792139077853365, "support": 1323}, "weighted avg": {"f1-score": 0.36688446129439917, "precision": 0.7053325889626367, "recall": 0.24792139077853365, "support": 1323}, "\u2205": {"f1-score": 0.5185770750988142, "precision": 0.9969604863221885, "recall": 0.3504273504273504, "support": 936}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 387}},
  "ppcr": 0.24867724867724866
}
```
</details>
