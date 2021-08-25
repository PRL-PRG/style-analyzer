# Train report for javascript / file:///tmp/top-repos-quality-repos-io60klpb/academic_projects.git HEAD 16859c4d586c7d1200631e4aadee46a7bb518ecd

### Classification report

PPCR: 0.829

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.957| 0.995| 0.790| 0.976| 0.866| 31278| 39358| 0.795 |
| `∅` | 0.996| 0.978| 0.918| 0.987| 0.956| 30219| 32204| 0.938 |
| `"` | 1.000| 1.000| 0.970| 1.000| 0.985| 9066| 9351| 0.970 |
| `⏎` | 0.992| 0.875| 0.530| 0.930| 0.691| 3210| 5301| 0.606 |
| `⏎⇥⁻` | 0.934| 0.967| 0.908| 0.950| 0.921| 2512| 2674| 0.939 |
| `⏎⇥⁺` | 0.936| 0.898| 0.748| 0.917| 0.831| 2448| 2940| 0.833 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 102| 115| 0.887 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 88| 2010| 0.044 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 48| 397| 0.121 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 370| 0.097 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 283| 0.021 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 263| 0.000 |
| `micro avg` | 0.977| 0.977| 0.810| 0.977| 0.886| 79013| 95266| 0.829 |
| `weighted avg` | 0.974| 0.977| 0.810| 0.975| 0.867| 79013| 95266| 0.829 |
| `macro avg` | 0.485| 0.476| 0.405| 0.480| 0.437| 79013| 95266| 0.829 |

### Confusion matrix

|refusal|  ␣| ∅| "| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| ⏎⏎⇥⁻| ⏎⏎⇥⁺| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⇥⁻⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8080 |31109 |87 |0 |4 |44 |34 |0 |0 |0 |0 |0 |0 |
|1985 |654 |29565 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|285 |0 |0 |9066 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2091 |384 |8 |0 |2808 |8 |2 |0 |0 |0 |0 |0 |0 |
|492 |244 |4 |0 |2 |2198 |0 |0 |0 |0 |0 |0 |0 |
|162 |70 |10 |0 |0 |4 |2428 |0 |0 |0 |0 |0 |0 |
|1922 |26 |0 |0 |16 |44 |2 |0 |0 |0 |0 |0 |0 |
|349 |0 |0 |0 |0 |6 |42 |0 |0 |0 |0 |0 |0 |
|334 |0 |0 |0 |0 |36 |0 |0 |0 |0 |0 |0 |0 |
|277 |0 |0 |0 |0 |6 |0 |0 |0 |0 |0 |0 |0 |
|263 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|13 |6 |2 |0 |0 |2 |92 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Web Service Example with ASP.NET 4.5/Web Service Simple Example using ASP.NET4.5/obj/Release/Package/PackageTmp/Scripts/jquery-3.1.1.slim.js | 830 |
| Web Service Example with ASP.NET 4.5/Web Service Simple Example using ASP.NET4.5/Scripts/jquery-3.1.1.slim.js | 830 |
| Web Service Example with ASP.NET 4.5/Web Service Simple Example using ASP.NET4.5/Scripts/respond.matchmedia.addListener.js | 51 |
| Web Service Example with ASP.NET 4.5/Web Service Simple Example using ASP.NET4.5/obj/Release/Package/PackageTmp/Scripts/respond.matchmedia.addListener.js | 51 |
| Simple Web Form JavaScript and DOM/a3.js | 29 |
| Web Service Example with ASP.NET 4.5/Web Service Simple Example using ASP.NET4.5/Scripts/respond.js | 24 |
| Web Service Example with ASP.NET 4.5/Web Service Simple Example using ASP.NET4.5/obj/Release/Package/PackageTmp/Scripts/respond.js | 24 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 9066}, "macro avg": {"f1-score": 0.47993292730857334, "precision": 0.48465449665129373, "recall": 0.47601312580126337, "support": 79013}, "micro avg": {"f1-score": 0.9767253489932036, "precision": 0.9767253489932036, "recall": 0.9767253489932036, "support": 79013}, "weighted avg": {"f1-score": 0.9749031445754478, "precision": 0.9737663356430436, "recall": 0.9767253489932036, "support": 79013}, "\u2205": {"f1-score": 0.9872276483846731, "precision": 0.996259603720178, "recall": 0.978357986697111, "support": 30219}, "\u23ce": {"f1-score": 0.9298013245033112, "precision": 0.992226148409894, "recall": 0.874766355140187, "support": 3210}, "\u23ce\u21e5\u207a": {"f1-score": 0.9165971643035864, "precision": 0.9361158432708688, "recall": 0.8978758169934641, "support": 2448}, "\u23ce\u21e5\u207b": {"f1-score": 0.9499217527386541, "precision": 0.9338461538461539, "recall": 0.9665605095541401, "support": 2512}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 102}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 88}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9756472377726553, "precision": 0.9574062105684301, "recall": 0.9945968412302577, "support": 31278}},
  "cl_report_full": {"\"": {"f1-score": 0.9845251669653039, "precision": 1.0, "recall": 0.9695219762592237, "support": 9351}, "macro avg": {"f1-score": 0.43739705057871947, "precision": 0.48465449665129373, "recall": 0.4052766789098931, "support": 95266}, "micro avg": {"f1-score": 0.8856373975062973, "precision": 0.9767253489932036, "recall": 0.8100896437343859, "support": 95266}, "weighted avg": {"f1-score": 0.8673389655058292, "precision": 0.9407890949665013, "recall": 0.8100896437343859, "support": 95266}, "\u2205": {"f1-score": 0.9555591467356174, "precision": 0.996259603720178, "recall": 0.9180536579306918, "support": 32204}, "\u23ce": {"f1-score": 0.6906899520354199, "precision": 0.992226148409894, "recall": 0.5297113752122241, "support": 5301}, "\u23ce\u21e5\u207a": {"f1-score": 0.8313161875945538, "precision": 0.9361158432708688, "recall": 0.7476190476190476, "support": 2940}, "\u23ce\u21e5\u207b": {"f1-score": 0.9207432688661358, "precision": 0.9338461538461539, "recall": 0.9080029917726253, "support": 2674}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2010}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 370}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 397}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 283}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 263}, "\u2423": {"f1-score": 0.8659308847476027, "precision": 0.9574062105684301, "recall": 0.7904110981249047, "support": 39358}},
  "ppcr": 0.8293934877081015
}
```
</details>
