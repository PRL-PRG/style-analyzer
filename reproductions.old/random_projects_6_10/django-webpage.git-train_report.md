# Train report for javascript / file:///tmp/top-repos-quality-repos-1n4rdwaq/django-webpage.git HEAD bfeebd3080481690b8736fbd5fe5e8003912a5b2

### Classification report

PPCR: 0.560

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.956| 0.999| 0.657| 0.977| 0.779| 2014| 3059| 0.658 |
| `"` | 1.000| 1.000| 0.891| 1.000| 0.943| 623| 699| 0.891 |
| `␣` | 0.986| 0.778| 0.154| 0.870| 0.266| 266| 1347| 0.197 |
| `⏎␣⁺␣⁺` | 1.000| 0.904| 0.842| 0.949| 0.914| 218| 234| 0.932 |
| `⏎␣⁻␣⁻` | 1.000| 0.990| 0.859| 0.995| 0.924| 191| 220| 0.868 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 378| 0.026 |
| `micro avg` | 0.971| 0.971| 0.544| 0.971| 0.697| 3322| 5937| 0.560 |
| `weighted avg` | 0.969| 0.971| 0.544| 0.969| 0.643| 3322| 5937| 0.560 |
| `macro avg` | 0.824| 0.778| 0.567| 0.798| 0.638| 3322| 5937| 0.560 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1045 |2011 |3 |0 |0 |0 |0 |
|1081 |59 |207 |0 |0 |0 |0 |
|76 |0 |0 |623 |0 |0 |0 |
|368 |10 |0 |0 |0 |0 |0 |
|16 |21 |0 |0 |0 |197 |0 |
|29 |2 |0 |0 |0 |0 |189 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| webpersonal/core/static/core/js/jqBootstrapValidation.js | 89 |
| webpersonal/core/static/core/js/contact_me.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 623}, "macro avg": {"f1-score": 0.7984678794958212, "precision": 0.8236612096098543, "recall": 0.7783174060524813, "support": 3322}, "micro avg": {"f1-score": 0.9714027694160144, "precision": 0.9714027694160144, "recall": 0.9714027694160144, "support": 3322}, "weighted avg": {"f1-score": 0.9689473788830103, "precision": 0.9693237463867879, "recall": 0.9714027694160144, "support": 3322}, "\u2205": {"f1-score": 0.9769249453485548, "precision": 0.9562529719448407, "recall": 0.9985104270109235, "support": 2014}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9493975903614458, "precision": 1.0, "recall": 0.9036697247706422, "support": 218}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9947368421052631, "precision": 1.0, "recall": 0.9895287958115183, "support": 191}, "\u2423": {"f1-score": 0.8697478991596639, "precision": 0.9857142857142858, "recall": 0.7781954887218046, "support": 266}},
  "cl_report_full": {"\"": {"f1-score": 0.9425113464447806, "precision": 1.0, "recall": 0.8912732474964234, "support": 699}, "macro avg": {"f1-score": 0.6376535295938078, "precision": 0.8236612096098543, "recall": 0.5672206186577201, "support": 5937}, "micro avg": {"f1-score": 0.6970515174424885, "precision": 0.9714027694160144, "recall": 0.5435405086744147, "support": 5937}, "weighted avg": {"f1-score": 0.6430268758986296, "precision": 0.9105499383588364, "recall": 0.5435405086744147, "support": 5937}, "\u2205": {"f1-score": 0.7791553661371562, "precision": 0.9562529719448407, "recall": 0.6574043805165086, "support": 3059}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 378}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.91415313225058, "precision": 1.0, "recall": 0.8418803418803419, "support": 234}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9242053789731052, "precision": 1.0, "recall": 0.8590909090909091, "support": 220}, "\u2423": {"f1-score": 0.26589595375722547, "precision": 0.9857142857142858, "recall": 0.15367483296213807, "support": 1347}},
  "ppcr": 0.5595418561563079
}
```
</details>
