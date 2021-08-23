# Train report for javascript / file:///tmp/top-repos-quality-repos-5jzy_qny/winck-django.git HEAD f24e993748f43921124fbf013e9735fd46650b7f

### Classification report

PPCR: 0.705

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.976| 0.988| 0.793| 0.982| 0.875| 30303| 37736| 0.803 |
| `␣` | 0.965| 0.968| 0.762| 0.967| 0.852| 17983| 22848| 0.787 |
| `'` | 0.978| 1.000| 0.648| 0.989| 0.779| 4374| 6753| 0.648 |
| `⏎` | 0.960| 0.828| 0.306| 0.889| 0.464| 1570| 4251| 0.369 |
| `⏎⇥⁺` | 0.935| 0.885| 0.530| 0.909| 0.676| 844| 1410| 0.599 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.924| 0.872| 0.232| 0.897| 0.371| 250| 939| 0.266 |
| `⏎⏎` | 0.950| 0.750| 0.138| 0.838| 0.241| 228| 1242| 0.184 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 100| 1785| 0.056 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 81| 960| 0.084 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 858| 0.031 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 233| 0.026 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 114| 0.018 |
| `micro avg` | 0.971| 0.971| 0.684| 0.971| 0.803| 55768| 79129| 0.705 |
| `macro avg` | 0.557| 0.524| 0.284| 0.539| 0.355| 55768| 79129| 0.705 |
| `weighted avg` | 0.967| 0.971| 0.684| 0.969| 0.775| 55768| 79129| 0.705 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎⇥⁺| ⏎⏎| ⏎⇥⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎⇥⁻| ⏎⇥⁻⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|7433 |29936 |340 |0 |0 |0 |10 |0 |0 |0 |17 |0 |0 |
|4865 |548 |17414 |0 |0 |0 |18 |2 |0 |0 |1 |0 |0 |
|2379 |0 |0 |4374 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2681 |99 |166 |0 |1300 |0 |3 |2 |0 |0 |0 |0 |0 |
|1685 |0 |0 |100 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|566 |33 |55 |0 |8 |0 |747 |1 |0 |0 |0 |0 |0 |
|1014 |9 |28 |0 |17 |0 |3 |171 |0 |0 |0 |0 |0 |
|831 |19 |1 |0 |6 |0 |0 |1 |0 |0 |0 |0 |0 |
|879 |22 |33 |0 |8 |0 |18 |0 |0 |0 |0 |0 |0 |
|689 |18 |0 |0 |14 |0 |0 |0 |0 |0 |218 |0 |0 |
|227 |2 |0 |0 |1 |0 |0 |3 |0 |0 |0 |0 |0 |
|112 |1 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/ckeditor/galleriffic/js/jquery-1.3.2.e4af2b480520.js | 700 |
| static/js/conflict-detection.js | 150 |
| static/ckeditor/galleriffic/js/jquery.galleriffic.a996d2596c48.js | 86 |
| static/ckeditor/galleriffic/js/jquery.galleriffic.js | 86 |
| static/admin/js/inlines.js | 52 |
| static/admin/js/inlines.12d1af430335.js | 52 |
| static/admin/js/admin/DateTimeShortcuts.a9c6d180860b.js | 42 |
| static/admin/js/admin/DateTimeShortcuts.js | 42 |
| static/admin/js/prepopulate.min.85fd5e0fb706.js | 40 |
| static/admin/js/core.js | 26 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 100}, "\u0027": {"f1-score": 0.9886980108499095, "precision": 0.9776486365668305, "recall": 1.0, "support": 4374}, "macro avg": {"f1-score": 0.5392589436377565, "precision": 0.557278977277745, "recall": 0.5242787134475849, "support": 55768}, "micro avg": {"f1-score": 0.9711662602209152, "precision": 0.9711662602209152, "recall": 0.9711662602209152, "support": 55768}, "weighted avg": {"f1-score": 0.9689855339055746, "precision": 0.9672666125432752, "recall": 0.9711662602209152, "support": 55768}, "\u2205": {"f1-score": 0.9816691260862437, "precision": 0.9755270961644996, "recall": 0.9878889878889879, "support": 30303}, "\u23ce": {"f1-score": 0.8891928864569085, "precision": 0.9601181683899557, "recall": 0.8280254777070064, "support": 1570}, "\u23ce\u21e5\u207a": {"f1-score": 0.9093122337188071, "precision": 0.934918648310388, "recall": 0.8850710900473934, "support": 844}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.838235294117647, "precision": 0.95, "recall": 0.75, "support": 228}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 81}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8971193415637859, "precision": 0.923728813559322, "recall": 0.872, "support": 250}, "\u2423": {"f1-score": 0.9668804308597763, "precision": 0.9654063643419448, "recall": 0.9683590057276317, "support": 17983}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1785}, "\u0027": {"f1-score": 0.779193016834417, "precision": 0.9776486365668305, "recall": 0.6477121279431364, "support": 6753}, "macro avg": {"f1-score": 0.3548181811282747, "precision": 0.557278977277745, "recall": 0.2840517489197805, "support": 79129}, "micro avg": {"f1-score": 0.8029830166719794, "precision": 0.9711662602209152, "recall": 0.6844519708324381, "support": 79129}, "weighted avg": {"f1-score": 0.774902413122977, "precision": 0.9215222654141927, "recall": 0.6844519708324381, "support": 79129}, "\u2205": {"f1-score": 0.8750274030662205, "precision": 0.9755270961644996, "recall": 0.7933008267966928, "support": 37736}, "\u23ce": {"f1-score": 0.46387154326494195, "precision": 0.9601181683899557, "recall": 0.3058103975535168, "support": 4251}, "\u23ce\u21e5\u207a": {"f1-score": 0.6763241285649615, "precision": 0.934918648310388, "recall": 0.5297872340425532, "support": 1410}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 858}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}, "\u23ce\u23ce": {"f1-score": 0.24050632911392408, "precision": 0.95, "recall": 0.13768115942028986, "support": 1242}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 233}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 960}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.37106382978723407, "precision": 0.923728813559322, "recall": 0.2321618743343983, "support": 939}, "\u2423": {"f1-score": 0.8518319229075968, "precision": 0.9654063643419448, "recall": 0.7621673669467787, "support": 22848}},
  "ppcr": 0.7047732184154988
}
```
</details>
