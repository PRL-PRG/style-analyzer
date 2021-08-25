# Train report for javascript / file:///tmp/top-repos-quality-repos-aodm9vx5/hospital-management-system.git HEAD 580830e297d4dc7afd434653de86d8fe2e9c3d84

### Classification report

PPCR: 0.667

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.937| 0.999| 0.928| 0.967| 0.933| 15562| 16754| 0.929 |
| `␣` | 0.982| 0.658| 0.203| 0.788| 0.336| 1871| 6073| 0.308 |
| `⏎` | 0.940| 0.938| 0.425| 0.939| 0.586| 1048| 2311| 0.453 |
| `"` | 0.988| 1.000| 0.517| 0.994| 0.679| 1025| 1981| 0.517 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 265| 768| 0.345 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 136| 783| 0.174 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 766| 0.016 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 310| 0.035 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 155| 0.052 |
| `weighted avg` | 0.924| 0.943| 0.629| 0.929| 0.681| 19938| 29901| 0.667 |
| `macro avg` | 0.427| 0.400| 0.230| 0.410| 0.282| 19938| 29901| 0.667 |
| `micro avg` | 0.943| 0.943| 0.629| 0.943| 0.754| 19938| 29901| 0.667 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1192 |15553 |5 |4 |0 |0 |0 |0 |0 |0 |
|4202 |595 |1232 |44 |0 |0 |0 |0 |0 |0 |
|1263 |62 |3 |983 |0 |0 |0 |0 |0 |0 |
|956 |0 |0 |0 |1025 |0 |0 |0 |0 |0 |
|754 |0 |0 |0 |12 |0 |0 |0 |0 |0 |
|503 |261 |0 |4 |0 |0 |0 |0 |0 |0 |
|647 |121 |15 |0 |0 |0 |0 |0 |0 |0 |
|299 |0 |0 |11 |0 |0 |0 |0 |0 |0 |
|147 |8 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| react/src/views/Dashboard.js | 195 |
| react/src/serviceWorker.js | 67 |
| react/scripts/build.js | 63 |
| react/src/redux/actions/patient.js | 60 |
| react/config/webpack.config.js | 60 |
| react/scripts/start.js | 39 |
| react/src/redux/actions/auth.js | 38 |
| react/src/components/AddPatient.js | 34 |
| react/src/components/UpdatePatient.js | 34 |
| react/config/modules.js | 32 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9941804073714839, "precision": 0.9884281581485053, "recall": 1.0, "support": 1025}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "macro avg": {"f1-score": 0.4098274850981954, "precision": 0.4274221922508994, "recall": 0.39954113034090066, "support": 19938}, "micro avg": {"f1-score": 0.9425719731166616, "precision": 0.9425719731166616, "recall": 0.9425719731166616, "support": 19938}, "weighted avg": {"f1-score": 0.9293203267175866, "precision": 0.9236231415552157, "recall": 0.9425719731166616, "support": 19938}, "\u2205": {"f1-score": 0.9671662210061563, "precision": 0.9369277108433735, "recall": 0.9994216681660455, "support": 15562}, "\u23ce": {"f1-score": 0.9388729703915949, "precision": 0.9397705544933078, "recall": 0.9379770992366412, "support": 1048}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 136}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 265}, "\u2423": {"f1-score": 0.7882277671145234, "precision": 0.9816733067729083, "recall": 0.6584714056654195, "support": 1871}},
  "cl_report_full": {"\"": {"f1-score": 0.6792577866136514, "precision": 0.9884281581485053, "recall": 0.5174154467440687, "support": 1981}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 766}, "macro avg": {"f1-score": 0.2815273396889779, "precision": 0.4274221922508994, "recall": 0.23043924466300453, "support": 29901}, "micro avg": {"f1-score": 0.754148357711832, "precision": 0.9425719731166616, "recall": 0.628507407779004, "support": 29901}, "weighted avg": {"f1-score": 0.6811093823536684, "precision": 0.862475328324403, "recall": 0.628507407779004, "support": 29901}, "\u2205": {"f1-score": 0.9326017868921269, "precision": 0.9369277108433735, "recall": 0.9283156261191358, "support": 16754}, "\u23ce": {"f1-score": 0.5856419422103069, "precision": 0.9397705544933078, "recall": 0.425356988316746, "support": 2311}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 310}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 783}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 155}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 768}, "\u2423": {"f1-score": 0.3362445414847161, "precision": 0.9816733067729083, "recall": 0.2028651407870904, "support": 6073}},
  "ppcr": 0.6668004414568075
}
```
</details>
