# Train report for javascript / file:///tmp/top-repos-quality-repos-fi0z6i7b/source-engine-model-loader.git HEAD e90f2a62fcae645742df40dae29fd65fe7529811

### Classification report

PPCR: 0.847

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.955| 0.977| 0.905| 0.966| 0.929| 6463| 6980| 0.926 |
| `∅` | 0.973| 0.960| 0.947| 0.966| 0.960| 5993| 6074| 0.987 |
| `⏎⏎⇥⁻` | 1.000| 0.922| 0.769| 0.959| 0.869| 166| 199| 0.834 |
| `⏎⏎` | 0.936| 0.936| 0.313| 0.936| 0.470| 157| 469| 0.335 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 56| 1114| 0.050 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 122| 0.000 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 203| 0.000 |
| `weighted avg` | 0.960| 0.964| 0.816| 0.962| 0.838| 12835| 15161| 0.847 |
| `micro avg` | 0.964| 0.964| 0.816| 0.964| 0.884| 12835| 15161| 0.847 |
| `macro avg` | 0.552| 0.542| 0.419| 0.547| 0.461| 12835| 15161| 0.847 |

### Confusion matrix

|refusal|  ␣| ∅| ⏎| ⏎⏎| '| ⏎⏎⇥⁺| ⏎⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|517 |6316 |147 |0 |0 |0 |0 |0 |
|81 |241 |5752 |0 |0 |0 |0 |0 |
|1058 |34 |13 |0 |9 |0 |0 |0 |
|312 |10 |0 |0 |147 |0 |0 |0 |
|122 |0 |0 |0 |0 |0 |0 |0 |
|203 |0 |0 |0 |0 |0 |0 |0 |
|33 |11 |1 |0 |1 |0 |0 |153 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| example/index.js | 143 |
| scripts/generate-header-read.js | 67 |
| src/SourceModelLoader.js | 60 |
| src/VTFLoader.js | 50 |
| example/SkinWeightsShaderMixin.js | 39 |
| src/VVDLoader.js | 37 |
| src/MDLLoader.js | 35 |
| src/VMTLoader.js | 30 |
| src/VTXLoader.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5468439199626893, "precision": 0.5520443572802204, "recall": 0.5421477202326792, "support": 12835}, "micro avg": {"f1-score": 0.9636151149201403, "precision": 0.9636151149201403, "recall": 0.9636151149201403, "support": 12835}, "weighted avg": {"f1-score": 0.9615043785304386, "precision": 0.9596020292263389, "recall": 0.9636151149201403, "support": 12835}, "\u2205": {"f1-score": 0.9662355115068033, "precision": 0.9727718586166074, "recall": 0.9597864174870683, "support": 5993}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u23ce": {"f1-score": 0.9363057324840764, "precision": 0.9363057324840764, "recall": 0.9363057324840764, "support": 157}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.9592476489028214, "precision": 1.0, "recall": 0.9216867469879518, "support": 166}, "\u2423": {"f1-score": 0.9661185468451242, "precision": 0.9552329098608591, "recall": 0.977255144669658, "support": 6463}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "macro avg": {"f1-score": 0.4611490443482742, "precision": 0.5520443572802204, "recall": 0.4191621822111891, "support": 15161}, "micro avg": {"f1-score": 0.8835547935419346, "precision": 0.9636151149201403, "recall": 0.8157773233955544, "support": 15161}, "weighted avg": {"f1-score": 0.8383035403356712, "precision": 0.871596159132056, "recall": 0.8157773233955544, "support": 15161}, "\u2205": {"f1-score": 0.9597063485442563, "precision": 0.9727718586166074, "recall": 0.9469871583799803, "support": 6074}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1114}, "\u23ce\u23ce": {"f1-score": 0.4696485623003195, "precision": 0.9363057324840764, "recall": 0.31343283582089554, "support": 469}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 203}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.8693181818181818, "precision": 1.0, "recall": 0.7688442211055276, "support": 199}, "\u2423": {"f1-score": 0.9293702177751619, "precision": 0.9552329098608591, "recall": 0.9048710601719198, "support": 6980}},
  "ppcr": 0.8465800408944001
}
```
</details>
