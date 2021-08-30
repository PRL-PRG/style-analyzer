# Test report for javascript / file:///tmp/top-repos-quality-repos-h5mm5sj6/toscani.git HEAD 98bd486591029911cfe1a0aba946e25bef0fcc44

### Classification report

PPCR: 0.882

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.943| 0.973| 0.966| 0.957| 0.954| 1282| 1291| 0.993 |
| `␣` | 0.713| 0.950| 0.927| 0.814| 0.806| 497| 509| 0.976 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 132| 139| 0.950 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 70| 282| 0.248 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 38| 0.316 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `micro avg` | 0.863| 0.863| 0.761| 0.863| 0.809| 1993| 2259| 0.882 |
| `macro avg` | 0.237| 0.275| 0.270| 0.253| 0.251| 1993| 2259| 0.882 |
| `weighted avg` | 0.784| 0.863| 0.761| 0.819| 0.727| 1993| 2259| 0.882 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| "| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|9 |1247 |35 |0 |0 |0 |0 |0 |
|12 |17 |472 |0 |0 |7 |0 |1 |
|7 |32 |100 |0 |0 |0 |0 |0 |
|26 |1 |11 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |
|212 |26 |44 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 70}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "macro avg": {"f1-score": 0.2531264128352094, "precision": 0.23650653375050684, "recall": 0.2746281567273039, "support": 1993}, "micro avg": {"f1-score": 0.8625188158554942, "precision": 0.8625188158554943, "recall": 0.8625188158554943, "support": 1993}, "weighted avg": {"f1-score": 0.8189551700839448, "precision": 0.784100225128286, "recall": 0.8625188158554943, "support": 1993}, "\u2205": {"f1-score": 0.9573896353166987, "precision": 0.9425547996976569, "recall": 0.9726989079563183, "support": 1282}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 132}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.814495254529767, "precision": 0.7129909365558912, "recall": 0.9496981891348089, "support": 497}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 282}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "macro avg": {"f1-score": 0.25146313349753147, "precision": 0.23650653375050684, "recall": 0.27046090586332155, "support": 2259}, "micro avg": {"f1-score": 0.8085606773283162, "precision": 0.8625188158554943, "recall": 0.7609561752988048, "support": 2259}, "weighted avg": {"f1-score": 0.7268986893740275, "precision": 0.6993141359524674, "recall": 0.7609561752988048, "support": 2259}, "\u2205": {"f1-score": 0.9540933435348126, "precision": 0.9425547996976569, "recall": 0.9659178931061193, "support": 1291}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 139}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8061485909479077, "precision": 0.7129909365558912, "recall": 0.9273084479371316, "support": 509}},
  "ppcr": 0.882248782647189
}
```
</details>
