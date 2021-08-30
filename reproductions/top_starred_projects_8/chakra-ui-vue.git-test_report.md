# Test report for javascript / file:///tmp/top-repos-quality-repos-my3guemq/chakra-ui-vue.git HEAD 8984d3cb11cb5e494fb8abd01046c6f2b77d6118

### Classification report

PPCR: 0.949

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 0.991| 0.971| 0.987| 0.977| 7086| 7235| 0.979 |
| `␣` | 0.948| 0.965| 0.934| 0.957| 0.941| 4824| 4987| 0.967 |
| `'` | 0.999| 0.997| 0.987| 0.998| 0.993| 1968| 1988| 0.990 |
| `⏎` | 0.959| 0.880| 0.667| 0.918| 0.787| 1140| 1504| 0.758 |
| `⏎␣⁺␣⁺` | 0.936| 0.912| 0.899| 0.924| 0.917| 799| 811| 0.985 |
| `⏎␣⁻␣⁻` | 0.929| 0.954| 0.922| 0.942| 0.926| 768| 795| 0.966 |
| `⏎⏎` | 0.963| 0.691| 0.361| 0.805| 0.525| 188| 360| 0.522 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.968| 0.968| 0.918| 0.968| 0.938| 16773| 17680| 0.949 |
| `macro avg` | 0.840| 0.799| 0.718| 0.816| 0.758| 16773| 17680| 0.949 |
| `micro avg` | 0.968| 0.968| 0.918| 0.968| 0.943| 16773| 17680| 0.949 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|149 |7023 |54 |0 |0 |0 |7 |2 |
|163 |28 |4657 |0 |39 |50 |49 |1 |
|20 |1 |4 |1963 |0 |0 |0 |0 |
|364 |44 |91 |0 |1003 |0 |0 |2 |
|12 |19 |49 |2 |0 |729 |0 |0 |
|27 |17 |17 |0 |1 |0 |733 |0 |
|172 |16 |39 |0 |3 |0 |0 |130 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.9982201881515382, "precision": 0.9989821882951654, "recall": 0.9974593495934959, "support": 1968}, "macro avg": {"f1-score": 0.8162357978612338, "precision": 0.8395584196442454, "recall": 0.7990101874764348, "support": 16773}, "micro avg": {"f1-score": 0.9681034996720921, "precision": 0.9681034996720921, "recall": 0.9681034996720921, "support": 16773}, "weighted avg": {"f1-score": 0.9676922664520341, "precision": 0.9681011417179047, "recall": 0.9681034996720921, "support": 16773}, "\u2205": {"f1-score": 0.9867921877195447, "precision": 0.9825125909345271, "recall": 0.9911092294665538, "support": 7086}, "\u23ce": {"f1-score": 0.9176578225068619, "precision": 0.9588910133843213, "recall": 0.8798245614035087, "support": 1140}, "\u23ce\u23ce": {"f1-score": 0.804953560371517, "precision": 0.9629629629629629, "recall": 0.6914893617021277, "support": 188}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.923954372623574, "precision": 0.9358151476251605, "recall": 0.9123904881101377, "support": 799}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9415542710340398, "precision": 0.9290240811153359, "recall": 0.9544270833333334, "support": 768}, "\u2423": {"f1-score": 0.9567539804827941, "precision": 0.9482793728364896, "recall": 0.9653814262023217, "support": 4824}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.9931697444978497, "precision": 0.9989821882951654, "recall": 0.9874245472837022, "support": 1988}, "macro avg": {"f1-score": 0.7581428577169991, "precision": 0.8395584196442454, "recall": 0.717606592794192, "support": 17680}, "micro avg": {"f1-score": 0.942617478884277, "precision": 0.9681034996720921, "recall": 0.9184389140271493, "support": 17680}, "weighted avg": {"f1-score": 0.9380286169623963, "precision": 0.9677535858645391, "recall": 0.9184389140271493, "support": 17680}, "\u2205": {"f1-score": 0.9765695612876311, "precision": 0.9825125909345271, "recall": 0.9706979958534899, "support": 7235}, "\u23ce": {"f1-score": 0.7866666666666666, "precision": 0.9588910133843213, "recall": 0.6668882978723404, "support": 1504}, "\u23ce\u23ce": {"f1-score": 0.5252525252525253, "precision": 0.9629629629629629, "recall": 0.3611111111111111, "support": 360}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9169811320754717, "precision": 0.9358151476251605, "recall": 0.8988902589395807, "support": 811}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9255050505050505, "precision": 0.9290240811153359, "recall": 0.9220125786163522, "support": 795}, "\u2423": {"f1-score": 0.9409981814507982, "precision": 0.9482793728364896, "recall": 0.9338279526769601, "support": 4987}},
  "ppcr": 0.9486990950226244
}
```
</details>