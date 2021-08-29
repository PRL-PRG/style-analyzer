# Train report for javascript / file:///tmp/top-repos-quality-repos-3b7jc6z7/disa_dj_project.git HEAD 37db0a60fe4fcdd32595ce32652cfd30acbdc61d

### Classification report

PPCR: 0.863

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.989| 0.992| 0.981| 0.991| 0.985| 70520| 71348| 0.988 |
| `␣` | 0.971| 0.987| 0.908| 0.979| 0.938| 36952| 40159| 0.920 |
| `'` | 0.979| 1.000| 0.927| 0.989| 0.952| 13703| 14778| 0.927 |
| `⏎␣⁻␣⁻` | 0.918| 0.934| 0.874| 0.926| 0.896| 4085| 4364| 0.936 |
| `⏎` | 0.984| 0.882| 0.279| 0.930| 0.435| 2419| 7642| 0.317 |
| `⏎␣⁺␣⁺` | 0.943| 0.388| 0.031| 0.550| 0.059| 340| 4308| 0.079 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 302| 366| 0.825 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 296| 1179| 0.251 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 102| 407| 0.251 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 4657| 0.008 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 52| 0.000 |
| `macro avg` | 0.526| 0.471| 0.364| 0.488| 0.388| 128758| 149260| 0.863 |
| `micro avg` | 0.980| 0.980| 0.846| 0.980| 0.908| 128758| 149260| 0.863 |
| `weighted avg` | 0.975| 0.980| 0.846| 0.977| 0.868| 128758| 149260| 0.863 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|828 |69964 |544 |0 |0 |0 |0 |12 |0 |0 |0 |0 |
|3207 |414 |36482 |0 |23 |0 |2 |31 |0 |0 |0 |0 |
|1075 |0 |0 |13703 |0 |0 |0 |0 |0 |0 |0 |0 |
|5223 |22 |257 |0 |2134 |0 |0 |6 |0 |0 |0 |0 |
|4618 |0 |27 |0 |12 |0 |0 |0 |0 |0 |0 |0 |
|3968 |107 |100 |0 |0 |0 |132 |1 |0 |0 |0 |0 |
|279 |187 |83 |0 |0 |0 |0 |3815 |0 |0 |0 |0 |
|883 |0 |0 |296 |0 |0 |0 |0 |0 |0 |0 |0 |
|305 |8 |88 |0 |0 |0 |6 |0 |0 |0 |0 |0 |
|64 |6 |7 |0 |0 |0 |0 |289 |0 |0 |0 |0 |
|52 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| disa_app/static/lib/select2-dist/js/select2.full.js | 531 |
| disa_app/static/lib/slickgrid/lib/jquery.event.drag-2.3.0.js | 429 |
| disa_app/static/lib/select2-dist/js/select2.js | 396 |
| disa_app/static/lib/bootstrap-4.5.0-dist/js/bootstrap.bundle.js | 252 |
| disa_app/static/js/browse_tabulator.js | 233 |
| disa_app/static/lib/bootstrap-4.1.2-dist/js/bootstrap.bundle.js | 224 |
| disa_app/static/js/browse.js | 124 |
| disa_app/static/js/editor/reference.locations.mgmt.js | 66 |
| disa_app/static/lib/tinymce/plugins/quickbars/plugin.js | 59 |
| disa_app/static/js/hilitor.js | 50 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 296}, "\u0027": {"f1-score": 0.9893148509132915, "precision": 0.9788556325451818, "recall": 1.0, "support": 13703}, "macro avg": {"f1-score": 0.48775481481287386, "precision": 0.5258201624204649, "recall": 0.4712471865070673, "support": 128758}, "micro avg": {"f1-score": 0.9803662685037047, "precision": 0.9803662685037047, "recall": 0.9803662685037047, "support": 128758}, "weighted avg": {"f1-score": 0.9771703651697806, "precision": 0.974759774234655, "recall": 0.9803662685037047, "support": 128758}, "\u2205": {"f1-score": 0.9907950264820007, "precision": 0.9894778525767947, "recall": 0.992115711854793, "support": 70520}, "\u23ce": {"f1-score": 0.9302528334786399, "precision": 0.9838635315813739, "recall": 0.882182720132286, "support": 2419}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.55, "precision": 0.9428571428571428, "recall": 0.38823529411764707, "support": 340}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 102}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9260832625318607, "precision": 0.9183919114106885, "recall": 0.9339045287637698, "support": 4085}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 302}, "\u2423": {"f1-score": 0.9788569895358198, "precision": 0.9705757156539321, "recall": 0.9872807967092444, "support": 36952}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1179}, "\u0027": {"f1-score": 0.9523577857316607, "precision": 0.9788556325451818, "recall": 0.9272567329814589, "support": 14778}, "macro avg": {"f1-score": 0.387816575223356, "precision": 0.5258201624204649, "recall": 0.363671157269092, "support": 149260}, "micro avg": {"f1-score": 0.9080707004582437, "precision": 0.9803662685037047, "recall": 0.8457054803698245, "support": 149260}, "weighted avg": {"f1-score": 0.8678195605235356, "precision": 0.9354717975982698, "recall": 0.8457054803698245, "support": 149260}, "\u2205": {"f1-score": 0.9850199921157853, "precision": 0.9894778525767947, "recall": 0.9806021191904468, "support": 71348}, "\u23ce": {"f1-score": 0.43502191417796343, "precision": 0.9838635315813739, "recall": 0.279246270609788, "support": 7642}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4657}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 52}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.05935251798561151, "precision": 0.9428571428571428, "recall": 0.03064066852367688, "support": 4308}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 407}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8957501760976755, "precision": 0.9183919114106885, "recall": 0.8741979835013749, "support": 4364}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 366}, "\u2423": {"f1-score": 0.9384799413482191, "precision": 0.9705757156539321, "recall": 0.9084389551532658, "support": 40159}},
  "ppcr": 0.8626423690205012
}
```
</details>
