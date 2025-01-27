# Test report for javascript / file:///tmp/top-repos-quality-repos-9uv6uh29/backbone-forms.git HEAD 6ec01c32c3c40d44de8871e98c44a1306854a823

### Classification report

PPCR: 0.944

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.949| 0.985| 0.984| 0.966| 0.966| 10196| 10204| 0.999 |
| `␣` | 0.899| 0.963| 0.958| 0.930| 0.928| 4188| 4209| 0.995 |
| `'` | 0.946| 0.748| 0.611| 0.835| 0.743| 837| 1024| 0.817 |
| `⏎` | 0.942| 0.755| 0.538| 0.838| 0.685| 714| 1001| 0.713 |
| `⏎⏎` | 0.917| 0.732| 0.546| 0.814| 0.685| 410| 549| 0.747 |
| `⏎␣⁻␣⁻` | 0.931| 0.633| 0.419| 0.753| 0.578| 256| 387| 0.661 |
| `⏎␣⁺␣⁺` | 0.863| 0.612| 0.373| 0.716| 0.521| 237| 389| 0.609 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 42| 72| 0.583 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 31| 46| 0.674 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 29| 0.897 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 25| 0.800 |
| `⏎⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 23| 0.348 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 16| 0.438 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.926| 0.934| 0.881| 0.927| 0.891| 16972| 17974| 0.944 |
| `micro avg` | 0.934| 0.934| 0.881| 0.934| 0.907| 16972| 17974| 0.944 |
| `macro avg` | 0.430| 0.362| 0.295| 0.390| 0.340| 16972| 17974| 0.944 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎␣⁻␣⁻| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8 |10038 |151 |0 |1 |1 |5 |0 |0 |0 |0 |0 |0 |0 |
|21 |142 |4034 |0 |6 |0 |4 |2 |0 |0 |0 |0 |0 |0 |
|187 |115 |93 |626 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|287 |93 |65 |0 |539 |17 |0 |0 |0 |0 |0 |0 |0 |0 |
|139 |50 |39 |0 |21 |300 |0 |0 |0 |0 |0 |0 |0 |0 |
|152 |11 |80 |0 |0 |1 |145 |0 |0 |0 |0 |0 |0 |0 |
|131 |92 |2 |0 |0 |0 |0 |162 |0 |0 |0 |0 |0 |0 |
|30 |6 |0 |36 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|15 |0 |22 |0 |0 |0 |9 |0 |0 |0 |0 |0 |0 |0 |
|3 |16 |0 |0 |0 |0 |0 |10 |0 |0 |0 |0 |0 |0 |
|5 |15 |0 |0 |2 |3 |0 |0 |0 |0 |0 |0 |0 |0 |
|9 |1 |1 |0 |0 |5 |0 |0 |0 |0 |0 |0 |0 |0 |
|15 |3 |0 |0 |0 |0 |5 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u0027": {"f1-score": 0.8352234823215476, "precision": 0.945619335347432, "recall": 0.7479091995221028, "support": 837}, "macro avg": {"f1-score": 0.39022491649248064, "precision": 0.42980810438094896, "recall": 0.36179182143827254, "support": 16972}, "micro avg": {"f1-score": 0.9335375913268914, "precision": 0.9335375913268914, "recall": 0.9335375913268914, "support": 16972}, "weighted avg": {"f1-score": 0.927437076786208, "precision": 0.9262530232578796, "recall": 0.9335375913268914, "support": 16972}, "\u2205": {"f1-score": 0.9662142650880738, "precision": 0.9485919485919486, "recall": 0.9845037269517458, "support": 10196}, "\u23ce": {"f1-score": 0.838258164852255, "precision": 0.9423076923076923, "recall": 0.7549019607843137, "support": 714}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.8141112618724559, "precision": 0.9174311926605505, "recall": 0.7317073170731707, "support": 410}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7160493827160495, "precision": 0.8630952380952381, "recall": 0.6118143459915611, "support": 237}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7534883720930232, "precision": 0.9310344827586207, "recall": 0.6328125, "support": 256}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u2423": {"f1-score": 0.930028818443804, "precision": 0.8990416759527524, "recall": 0.9632282712511939, "support": 4188}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 72}, "\u0027": {"f1-score": 0.7425860023724793, "precision": 0.945619335347432, "recall": 0.611328125, "support": 1024}, "macro avg": {"f1-score": 0.3403095935773939, "precision": 0.42980810438094896, "recall": 0.2953164895143134, "support": 17974}, "micro avg": {"f1-score": 0.9067704458307103, "precision": 0.9335375913268914, "recall": 0.8814954934905975, "support": 17974}, "weighted avg": {"f1-score": 0.8906736880724729, "precision": 0.9221534980599991, "recall": 0.8814954934905975, "support": 17974}, "\u2205": {"f1-score": 0.965842393918984, "precision": 0.9485919485919486, "recall": 0.9837318698549589, "support": 10204}, "\u23ce": {"f1-score": 0.6853146853146853, "precision": 0.9423076923076923, "recall": 0.5384615384615384, "support": 1001}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.6849315068493151, "precision": 0.9174311926605505, "recall": 0.546448087431694, "support": 549}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5206463195691203, "precision": 0.8630952380952381, "recall": 0.37275064267352187, "support": 389}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5775401069518717, "precision": 0.9310344827586207, "recall": 0.4186046511627907, "support": 387}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u2423": {"f1-score": 0.9277828886844526, "precision": 0.8990416759527524, "recall": 0.9584224281301972, "support": 4209}},
  "ppcr": 0.9442528096138867
}
```
</details>
