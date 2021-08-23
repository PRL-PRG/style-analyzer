# Train report for javascript / file:///tmp/top-repos-quality-repos-8qd299gv/django-notes.git HEAD 49305b2696dc08fa8216d014b8406547d652aeda

### Classification report

PPCR: 0.885

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.975| 0.985| 0.971| 0.980| 0.973| 29971| 30403| 0.986 |
| `␣` | 0.935| 0.962| 0.928| 0.949| 0.932| 14218| 14740| 0.965 |
| `'` | 0.939| 1.000| 0.850| 0.969| 0.893| 5789| 6807| 0.850 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.987| 0.939| 0.922| 0.962| 0.953| 1515| 1543| 0.982 |
| `⏎` | 0.959| 0.579| 0.179| 0.722| 0.302| 1012| 3276| 0.309 |
| `"` | 1.000| 0.026| 0.017| 0.051| 0.033| 384| 603| 0.637 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 200| 1617| 0.124 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 833| 0.029 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 210| 0.110 |
| `macro avg` | 0.644| 0.499| 0.430| 0.515| 0.454| 53136| 60032| 0.885 |
| `weighted avg` | 0.956| 0.960| 0.850| 0.954| 0.864| 53136| 60032| 0.885 |
| `micro avg` | 0.960| 0.960| 0.850| 0.960| 0.902| 53136| 60032| 0.885 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| "| ⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|432 |29529 |437 |0 |0 |0 |5 |0 |0 |0 |
|522 |522 |13682 |0 |0 |0 |14 |0 |0 |0 |
|1018 |0 |0 |5789 |0 |0 |0 |0 |0 |0 |
|2264 |104 |322 |0 |586 |0 |0 |0 |0 |0 |
|1417 |33 |154 |0 |13 |0 |0 |0 |0 |0 |
|28 |79 |14 |0 |0 |0 |1422 |0 |0 |0 |
|809 |7 |7 |0 |10 |0 |0 |0 |0 |0 |
|219 |0 |0 |374 |0 |0 |0 |0 |10 |0 |
|187 |8 |13 |0 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| geodjango/static_root/mapwidgets/js/jquery.custom.magnific-popup.js | 388 |
| django-blog/static_root/admin/js/urlify.js | 155 |
| geodjango/static_root/admin/js/urlify.js | 155 |
| geodjango/static_root/admin/js/inlines.js | 92 |
| django-blog/static_root/admin/js/inlines.js | 86 |
| geodjango/static_root/mapwidgets/js/django_mw_base.js | 82 |
| django-blog/static_root/admin/js/core.js | 78 |
| geodjango/static_root/admin/js/core.js | 77 |
| geodjango/static_root/gis/js/OLMapWidget.js | 66 |
| geodjango/static_root/leaflet/leaflet.forms.js | 66 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.050761421319796954, "precision": 1.0, "recall": 0.026041666666666668, "support": 384}, "\u0027": {"f1-score": 0.9687081659973227, "precision": 0.9393152685380497, "recall": 1.0, "support": 5789}, "macro avg": {"f1-score": 0.5147175426424941, "precision": 0.6439569734971595, "recall": 0.4990289589240587, "support": 53136}, "micro avg": {"f1-score": 0.9601400180668473, "precision": 0.9601400180668473, "recall": 0.9601400180668473, "support": 53136}, "weighted avg": {"f1-score": 0.9537672321932479, "precision": 0.9562378068915318, "recall": 0.9601400180668473, "support": 53136}, "\u2205": {"f1-score": 0.9801669626408643, "precision": 0.9751337428175153, "recall": 0.9852524106636416, "support": 29971}, "\u23ce": {"f1-score": 0.7221195317313617, "precision": 0.9590834697217676, "recall": 0.5790513833992095, "support": 1012}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 200}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9621109607577808, "precision": 0.9868147120055517, "recall": 0.9386138613861386, "support": 1515}, "\u2423": {"f1-score": 0.9485908413353208, "precision": 0.9352655683915511, "recall": 0.9623013082008721, "support": 14218}},
  "cl_report_full": {"\"": {"f1-score": 0.032626427406199025, "precision": 1.0, "recall": 0.01658374792703151, "support": 603}, "\u0027": {"f1-score": 0.8926754047802622, "precision": 0.9393152685380497, "recall": 0.8504480681651241, "support": 6807}, "macro avg": {"f1-score": 0.45386921994328033, "precision": 0.6439569734971595, "recall": 0.4296627989629346, "support": 60032}, "micro avg": {"f1-score": 0.9016329704510109, "precision": 0.9601400180668473, "recall": 0.8498467484008528, "support": 60032}, "weighted avg": {"f1-score": 0.8641405258154702, "precision": 0.9177494876289971, "recall": 0.8498467484008528, "support": 60032}, "\u2205": {"f1-score": 0.9731894207794347, "precision": 0.9751337428175153, "recall": 0.9712528368910963, "support": 30403}, "\u23ce": {"f1-score": 0.3015178801131978, "precision": 0.9590834697217676, "recall": 0.17887667887667888, "support": 3276}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 210}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 833}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1617}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.953083109919571, "precision": 0.9868147120055517, "recall": 0.9215813350615684, "support": 1543}, "\u2423": {"f1-score": 0.9317307364908577, "precision": 0.9352655683915511, "recall": 0.9282225237449118, "support": 14740}},
  "ppcr": 0.8851279317697228
}
```
</details>
