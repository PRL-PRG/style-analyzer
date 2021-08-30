# Test report for javascript / file:///tmp/top-repos-quality-repos-ck818s4y/resumake.io.git HEAD 9f52bcc874b54e1449b9225ab9a117678238d01f

### Classification report

PPCR: 0.947

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.967| 0.930| 0.895| 0.948| 0.929| 3875| 4027| 0.962 |
| `␣` | 0.767| 0.879| 0.832| 0.819| 0.798| 1812| 1915| 0.946 |
| `'` | 0.957| 0.939| 0.887| 0.948| 0.920| 708| 750| 0.944 |
| `⏎` | 0.775| 0.786| 0.706| 0.780| 0.739| 373| 415| 0.899 |
| `⏎␣⁺␣⁺` | 0.845| 0.832| 0.817| 0.839| 0.831| 328| 334| 0.982 |
| `⏎␣⁻␣⁻` | 0.873| 0.779| 0.708| 0.823| 0.782| 290| 319| 0.909 |
| `⏎⏎` | 0.844| 0.755| 0.696| 0.797| 0.763| 200| 217| 0.922 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 60| 0.500 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 10| 0.900 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 8| 1.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 8| 0.500 |
| `weighted avg` | 0.891| 0.891| 0.844| 0.890| 0.863| 7637| 8063| 0.947 |
| `micro avg` | 0.891| 0.891| 0.844| 0.891| 0.867| 7637| 8063| 0.947 |
| `macro avg` | 0.548| 0.536| 0.504| 0.541| 0.524| 7637| 8063| 0.947 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|152 |3603 |261 |0 |0 |0 |0 |11 |0 |0 |0 |0 |
|103 |49 |1593 |0 |85 |48 |30 |7 |0 |0 |0 |0 |
|42 |1 |42 |665 |0 |0 |0 |0 |0 |0 |0 |0 |
|42 |14 |66 |0 |293 |0 |0 |0 |0 |0 |0 |0 |
|6 |14 |41 |0 |0 |273 |0 |0 |0 |0 |0 |0 |
|29 |32 |22 |0 |0 |0 |226 |10 |0 |0 |0 |0 |
|17 |8 |41 |0 |0 |0 |0 |151 |0 |0 |0 |0 |
|30 |0 |0 |30 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |1 |7 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4 |1 |1 |0 |0 |2 |0 |0 |0 |0 |0 |0 |
|1 |3 |3 |0 |0 |0 |3 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u0027": {"f1-score": 0.9479686386315039, "precision": 0.9568345323741008, "recall": 0.9392655367231638, "support": 708}, "macro avg": {"f1-score": 0.5413079127263111, "precision": 0.5479355144527934, "recall": 0.5363964788532564, "support": 7637}, "micro avg": {"f1-score": 0.8909257561869844, "precision": 0.8909257561869844, "recall": 0.8909257561869844, "support": 7637}, "weighted avg": {"f1-score": 0.8895529627909935, "precision": 0.8907147202725251, "recall": 0.8909257561869844, "support": 7637}, "\u2205": {"f1-score": 0.9480331535324299, "precision": 0.966988727858293, "recall": 0.9298064516129032, "support": 3875}, "\u23ce": {"f1-score": 0.7802929427430092, "precision": 0.7751322751322751, "recall": 0.7855227882037533, "support": 373}, "\u23ce\u23ce": {"f1-score": 0.7968337730870713, "precision": 0.8435754189944135, "recall": 0.755, "support": 200}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8387096774193548, "precision": 0.8452012383900929, "recall": 0.8323170731707317, "support": 328}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8233151183970856, "precision": 0.8725868725868726, "recall": 0.7793103448275862, "support": 290}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.8192337361789663, "precision": 0.7669715936446798, "recall": 0.8791390728476821, "support": 1812}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 60}, "\u0027": {"f1-score": 0.9204152249134949, "precision": 0.9568345323741008, "recall": 0.8866666666666667, "support": 750}, "macro avg": {"f1-score": 0.523873404230704, "precision": 0.5479355144527934, "recall": 0.5037215459614488, "support": 8063}, "micro avg": {"f1-score": 0.866751592356688, "precision": 0.8909257561869844, "recall": 0.8438546446731986, "support": 8063}, "weighted avg": {"f1-score": 0.8632939480686385, "precision": 0.8862493233628511, "recall": 0.8438546446731986, "support": 8063}, "\u2205": {"f1-score": 0.9294466658067845, "precision": 0.966988727858293, "recall": 0.8947107027563943, "support": 4027}, "\u23ce": {"f1-score": 0.7389659520807063, "precision": 0.7751322751322751, "recall": 0.7060240963855422, "support": 415}, "\u23ce\u23ce": {"f1-score": 0.7626262626262625, "precision": 0.8435754189944135, "recall": 0.695852534562212, "support": 217}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8310502283105022, "precision": 0.8452012383900929, "recall": 0.8173652694610778, "support": 334}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.782006920415225, "precision": 0.8725868725868726, "recall": 0.7084639498432602, "support": 319}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u2423": {"f1-score": 0.7980961923847696, "precision": 0.7669715936446798, "recall": 0.8318537859007833, "support": 1915}},
  "ppcr": 0.9471660672206375
}
```
</details>