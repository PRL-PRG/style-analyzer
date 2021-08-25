# Train report for javascript / file:///tmp/top-repos-quality-repos-x74wwibk/facetrack.git HEAD 1b8c77eb4fba41c94b36b251274b7c13c702ccc3

### Classification report

PPCR: 0.876

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.904| 0.978| 0.969| 0.939| 0.935| 54311| 54779| 0.991 |
| `␣` | 0.944| 0.851| 0.786| 0.895| 0.858| 37031| 40068| 0.924 |
| `'` | 0.858| 1.000| 0.990| 0.923| 0.919| 11444| 11559| 0.990 |
| `⏎` | 0.931| 0.824| 0.373| 0.874| 0.532| 2979| 6582| 0.453 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 1900| 1946| 0.976 |
| `⏎⏎` | 0.859| 0.809| 0.229| 0.833| 0.361| 1170| 4140| 0.283 |
| `⏎␣⁺␣⁺` | 0.946| 0.482| 0.041| 0.639| 0.079| 220| 2588| 0.085 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 193| 2670| 0.072 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 44| 233| 0.189 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 223| 0.000 |
| `micro avg` | 0.911| 0.911| 0.798| 0.911| 0.851| 109292| 124788| 0.876 |
| `weighted avg` | 0.895| 0.911| 0.798| 0.901| 0.813| 109292| 124788| 0.876 |
| `macro avg` | 0.544| 0.494| 0.339| 0.510| 0.368| 109292| 124788| 0.876 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|468 |53092 |1204 |0 |0 |15 |0 |0 |0 |0 |0 |
|3037 |5427 |31512 |0 |90 |1 |1 |0 |0 |0 |0 |
|115 |0 |0 |11444 |0 |0 |0 |0 |0 |0 |0 |
|3603 |12 |376 |0 |2454 |137 |0 |0 |0 |0 |0 |
|2970 |0 |133 |0 |91 |946 |0 |0 |0 |0 |0 |
|2368 |71 |43 |0 |0 |0 |106 |0 |0 |0 |0 |
|2477 |126 |65 |0 |0 |2 |0 |0 |0 |0 |0 |
|46 |0 |0 |1900 |0 |0 |0 |0 |0 |0 |0 |
|189 |2 |37 |0 |0 |0 |5 |0 |0 |0 |0 |
|223 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| websocket_client/src/main/resources/static/admin/plugins/jquery/jquery.slim.js | 8015 |
| websocket_client/src/main/resources/static/admin/plugins/bootstrap/js/bootstrap.bundle.js | 774 |
| websocket_client/src/main/resources/static/admin/plugins/select2/select2.full.js | 543 |
| websocket_client/src/main/resources/static/admin/plugins/select2/select2.js | 406 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1900}, "\u0027": {"f1-score": 0.9233500080684203, "precision": 0.8576139088729017, "recall": 1.0, "support": 11444}, "macro avg": {"f1-score": 0.5103815504808074, "precision": 0.5442893278925178, "recall": 0.49426494532359727, "support": 109292}, "micro avg": {"f1-score": 0.9108992423965158, "precision": 0.9108992423965158, "recall": 0.9108992423965158, "support": 109292}, "weighted avg": {"f1-score": 0.900831065578508, "precision": 0.8954798377317352, "recall": 0.9108992423965158, "support": 109292}, "\u2205": {"f1-score": 0.9393405932360824, "precision": 0.9040013621658437, "recall": 0.9775551913976911, "support": 54311}, "\u23ce": {"f1-score": 0.8742429640185251, "precision": 0.9313092979127134, "recall": 0.823766364551863, "support": 2979}, "\u23ce\u23ce": {"f1-score": 0.8331131660061647, "precision": 0.8592188919164396, "recall": 0.8085470085470086, "support": 1170}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6385542168674699, "precision": 0.9464285714285714, "recall": 0.4818181818181818, "support": 220}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 193}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8952145566114117, "precision": 0.9443212466287084, "recall": 0.8509627069212281, "support": 37031}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1946}, "\u0027": {"f1-score": 0.9190860538890896, "precision": 0.8576139088729017, "recall": 0.990051042477723, "support": 11559}, "macro avg": {"f1-score": 0.368475966369268, "precision": 0.5442893278925178, "recall": 0.33880132712051864, "support": 124788}, "micro avg": {"f1-score": 0.8505980861244019, "precision": 0.9108992423965158, "recall": 0.7977850434336635, "support": 124788}, "weighted avg": {"f1-score": 0.8130301898981346, "precision": 0.8767422721328684, "recall": 0.7977850434336635, "support": 124788}, "\u2205": {"f1-score": 0.9354676721669647, "precision": 0.9040013621658437, "recall": 0.9692035268989941, "support": 54779}, "\u23ce": {"f1-score": 0.5324943040034719, "precision": 0.9313092979127134, "recall": 0.37283500455788515, "support": 6582}, "\u23ce\u23ce": {"f1-score": 0.36099980919671815, "precision": 0.8592188919164396, "recall": 0.2285024154589372, "support": 4140}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.07851851851851852, "precision": 0.9464285714285714, "recall": 0.04095826893353941, "support": 2588}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 233}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2670}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 223}, "\u2423": {"f1-score": 0.8581933059179171, "precision": 0.9443212466287084, "recall": 0.7864630128781073, "support": 40068}},
  "ppcr": 0.8758213930826682
}
```
</details>
