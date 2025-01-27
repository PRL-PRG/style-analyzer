# Test report for javascript / file:///tmp/top-repos-quality-repos-da_p2t1f/plotdb.git HEAD ad11ce4247c7af1fdf64999ce328f62e61cbca11

### Classification report

PPCR: 0.983

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.986| 0.985| 0.986| 0.986| 95797| 95818| 1.000 |
| `␣` | 0.960| 0.977| 0.973| 0.969| 0.967| 60512| 60747| 0.996 |
| `⏎` | 0.841| 0.910| 0.834| 0.874| 0.837| 9301| 10152| 0.916 |
| `'` | 0.865| 0.858| 0.804| 0.862| 0.834| 8304| 8860| 0.937 |
| `"` | 0.784| 0.776| 0.726| 0.780| 0.754| 4897| 5236| 0.935 |
| `⏎␣⁻␣⁻` | 0.737| 0.949| 0.943| 0.830| 0.827| 4184| 4211| 0.994 |
| `⏎␣⁺␣⁺` | 0.721| 0.955| 0.925| 0.822| 0.810| 4105| 4240| 0.968 |
| `⏎⏎` | 0.924| 0.607| 0.484| 0.733| 0.635| 3151| 3952| 0.797 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1461| 1739| 0.840 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.978| 0.065| 0.064| 0.121| 0.120| 1363| 1382| 0.986 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 165| 297| 0.556 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 62| 62| 1.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 49| 49| 1.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 48| 48| 1.000 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 40| 40| 1.000 |
| `⏎⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 43| 0.651 |
| `␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 5| 1.000 |
| `␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 3| 1.000 |
| `macro avg` | 0.433| 0.393| 0.374| 0.388| 0.376| 193475| 196884| 0.983 |
| `weighted avg` | 0.939| 0.945| 0.929| 0.938| 0.927| 193475| 196884| 0.983 |
| `micro avg` | 0.945| 0.945| 0.929| 0.945| 0.937| 193475| 196884| 0.983 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁺␣⁺| ⏎⏎␣⁻␣⁻| ␣␣␣| ␣␣| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|21 |94416 |1308 |4 |0 |43 |25 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|235 |837 |59118 |230 |0 |200 |124 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|851 |112 |566 |8468 |0 |19 |2 |0 |134 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|556 |95 |42 |1 |7127 |0 |0 |1039 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|135 |58 |64 |8 |53 |3920 |1 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|27 |138 |12 |40 |0 |19 |3970 |0 |3 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|339 |33 |3 |4 |1056 |2 |0 |3799 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|801 |24 |231 |980 |0 |2 |1 |0 |1913 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|278 |33 |182 |67 |3 |1173 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|19 |14 |30 |91 |0 |13 |1123 |0 |4 |0 |88 |0 |0 |0 |0 |0 |0 |0 |0 |
|132 |1 |4 |144 |0 |1 |4 |0 |11 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |1 |2 |8 |0 |37 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |2 |0 |1 |0 |0 |45 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |1 |2 |0 |1 |58 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|15 |0 |0 |20 |0 |8 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |6 |0 |0 |34 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |1 |1 |0 |0 |0 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.7800821355236139, "precision": 0.7844311377245509, "recall": 0.7757810904635492, "support": 4897}, "\u0027": {"f1-score": 0.8616333192286768, "precision": 0.8650321640975847, "recall": 0.8582610789980732, "support": 8304}, "macro avg": {"f1-score": 0.3875087399242882, "precision": 0.4331051344280827, "recall": 0.3934715195593571, "support": 193475}, "micro avg": {"f1-score": 0.9449231166817418, "precision": 0.9449231166817418, "recall": 0.9449231166817418, "support": 193475}, "weighted avg": {"f1-score": 0.937902888714288, "precision": 0.9390397122784998, "recall": 0.9449231166817418, "support": 193475}, "\u2205": {"f1-score": 0.9857487393115545, "precision": 0.9859134339268, "recall": 0.9855840997108469, "support": 95797}, "\u23ce": {"f1-score": 0.874116129032258, "precision": 0.8405797101449275, "recall": 0.9104397376626169, "support": 9301}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u23ce\u23ce": {"f1-score": 0.7328098065504693, "precision": 0.9241545893719807, "recall": 0.6071088543319582, "support": 3151}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 165}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8215445876558733, "precision": 0.7208532548731151, "recall": 0.9549330085261876, "support": 4105}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1461}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8295893845993104, "precision": 0.7369593465750882, "recall": 0.9488527724665392, "support": 4184}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.1211286992429456, "precision": 0.9777777777777777, "recall": 0.06456346294937637, "support": 1363}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}, "\u2423": {"f1-score": 0.9685045174924846, "precision": 0.9601910052136627, "recall": 0.9769632469592808, "support": 60512}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}},
  "cl_report_full": {"\"": {"f1-score": 0.7538446274431987, "precision": 0.7844311377245509, "recall": 0.725553857906799, "support": 5236}, "\u0027": {"f1-score": 0.8336160009357272, "precision": 0.8650321640975847, "recall": 0.8044018058690745, "support": 8860}, "macro avg": {"f1-score": 0.376073660711405, "precision": 0.4331051344280827, "recall": 0.3743144861524906, "support": 196884}, "micro avg": {"f1-score": 0.9366711155628537, "precision": 0.9449231166817418, "recall": 0.9285619958960606, "support": 196884}, "weighted avg": {"f1-score": 0.9274033316943581, "precision": 0.9359079938913817, "recall": 0.9285619958960606, "support": 196884}, "\u2205": {"f1-score": 0.9856406883700537, "precision": 0.9859134339268, "recall": 0.9853680936775971, "support": 95818}, "\u23ce": {"f1-score": 0.8373380796993968, "precision": 0.8405797101449275, "recall": 0.8341213553979512, "support": 10152}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u23ce\u23ce": {"f1-score": 0.6353370973098639, "precision": 0.9241545893719807, "recall": 0.4840587044534413, "support": 3952}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 297}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8100847282496384, "precision": 0.7208532548731151, "recall": 0.9245283018867925, "support": 4240}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1739}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8272556782663055, "precision": 0.7369593465750882, "recall": 0.9427689384944193, "support": 4211}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.11956521739130434, "precision": 0.9777777777777777, "recall": 0.06367583212735166, "support": 1382}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}, "\u2423": {"f1-score": 0.9666437751398018, "precision": 0.9601910052136627, "recall": 0.973183860931404, "support": 60747}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}},
  "ppcr": 0.9826852359765141
}
```
</details>
