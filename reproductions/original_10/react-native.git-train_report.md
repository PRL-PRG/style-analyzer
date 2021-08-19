# Train report for javascript / file:///tmp/top-repos-quality-repos-o_j0l67g/react-native.git HEAD 9b4f8e01442356f820e135fae3849063b2b8c92c

### Classification report

PPCR: 0.936

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.985| 0.990| 0.973| 0.987| 0.979| 293800| 298922| 0.983 |
| `␣` | 0.953| 0.990| 0.949| 0.971| 0.951| 157892| 164659| 0.959 |
| `⏎` | 0.886| 0.929| 0.708| 0.907| 0.787| 34859| 45717| 0.762 |
| `⏎␣⁺␣⁺` | 0.961| 0.767| 0.580| 0.853| 0.723| 21086| 27880| 0.756 |
| `⏎␣⁻␣⁻` | 0.919| 0.815| 0.683| 0.864| 0.783| 19094| 22793| 0.838 |
| `'` | 0.967| 0.981| 0.926| 0.974| 0.946| 18642| 19739| 0.944 |
| `"` | 0.945| 0.907| 0.840| 0.926| 0.889| 6761| 7299| 0.926 |
| `⏎⏎` | 0.904| 0.747| 0.530| 0.818| 0.668| 4756| 6701| 0.710 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1253| 2174| 0.576 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 415| 635| 0.654 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 180| 461| 0.390 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 168| 207| 0.812 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 149| 0.215 |
| `weighted avg` | 0.961| 0.965| 0.903| 0.962| 0.926| 558938| 597336| 0.936 |
| `macro avg` | 0.578| 0.548| 0.476| 0.562| 0.518| 558938| 597336| 0.936 |
| `micro avg` | 0.965| 0.965| 0.903| 0.965| 0.933| 558938| 597336| 0.936 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| "| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5122 |290866 |2255 |79 |251 |349 |0 |0 |0 |0 |0 |0 |0 |0 |
|6767 |532 |156290 |816 |202 |51 |0 |0 |1 |0 |0 |0 |0 |0 |
|10858 |341 |1700 |32376 |69 |39 |0 |0 |334 |0 |0 |0 |0 |0 |
|6794 |2278 |2497 |143 |16168 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3699 |1024 |766 |1703 |4 |15566 |0 |0 |30 |1 |0 |0 |0 |0 |
|1097 |0 |0 |0 |0 |0 |18284 |358 |0 |0 |0 |0 |0 |0 |
|538 |0 |0 |0 |0 |0 |628 |6133 |0 |0 |0 |0 |0 |0 |
|1945 |8 |114 |1075 |0 |8 |0 |0 |3551 |0 |0 |0 |0 |0 |
|921 |194 |4 |259 |0 |796 |0 |0 |0 |0 |0 |0 |0 |0 |
|220 |136 |153 |4 |122 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|281 |10 |2 |62 |0 |106 |0 |0 |0 |0 |0 |0 |0 |0 |
|39 |0 |136 |18 |0 |1 |0 |0 |13 |0 |0 |0 |0 |0 |
|117 |3 |0 |0 |0 |29 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Libraries/Renderer/implementations/ReactNativeRenderer-profiling.fb.js | 1330 |
| Libraries/Renderer/implementations/ReactNativeRenderer-profiling.js | 1328 |
| Libraries/Renderer/implementations/ReactFabric-profiling.fb.js | 1288 |
| Libraries/Renderer/implementations/ReactFabric-profiling.js | 1286 |
| Libraries/Renderer/implementations/ReactNativeRenderer-prod.fb.js | 1276 |
| Libraries/Renderer/implementations/ReactNativeRenderer-prod.js | 1274 |
| Libraries/Renderer/implementations/ReactFabric-prod.fb.js | 1234 |
| Libraries/Renderer/implementations/ReactFabric-prod.js | 1232 |
| Libraries/Utilities/MatrixMath.js | 362 |
| Libraries/Pressability/Pressability.js | 213 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9255961364322366, "precision": 0.9448467108303805, "recall": 0.9071143321993788, "support": 6761}, "\u0027": {"f1-score": 0.9737444746232092, "precision": 0.9667935702199661, "recall": 0.980796051925759, "support": 18642}, "macro avg": {"f1-score": 0.5615145334810211, "precision": 0.5784484016112773, "recall": 0.5480906486926748, "support": 558938}, "micro avg": {"f1-score": 0.9647474317366148, "precision": 0.9647474317366148, "recall": 0.9647474317366148, "support": 558938}, "weighted avg": {"f1-score": 0.9622592031085054, "precision": 0.9612114449914693, "recall": 0.9647474317366148, "support": 558938}, "\u2205": {"f1-score": 0.9873385925131367, "precision": 0.9846779872169863, "recall": 0.9900136147038802, "support": 293800}, "\u23ce": {"f1-score": 0.906966972014455, "precision": 0.8861639523744355, "recall": 0.928770188473565, "support": 34859}, "\u23ce\u23ce": {"f1-score": 0.8177317213586643, "precision": 0.9037923135657928, "recall": 0.7466358284272497, "support": 4756}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 168}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8531475911561396, "precision": 0.961465271170314, "recall": 0.7667646779853932, "support": 21086}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 415}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8638419489996948, "precision": 0.9186190616701092, "recall": 0.8152299151565937, "support": 19094}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1253}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 180}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u2423": {"f1-score": 0.9713214981557383, "precision": 0.9534703538986194, "recall": 0.9898538241329516, "support": 157892}},
  "cl_report_full": {"\"": {"f1-score": 0.889485134155185, "precision": 0.9448467108303805, "recall": 0.8402520893273051, "support": 7299}, "\u0027": {"f1-score": 0.9461074745802179, "precision": 0.9667935702199661, "recall": 0.9262880591721971, "support": 19739}, "macro avg": {"f1-score": 0.5175372450170025, "precision": 0.5784484016112773, "recall": 0.47613157433334974, "support": 597336}, "micro avg": {"f1-score": 0.9327097210522766, "precision": 0.9647474317366148, "recall": 0.9027314610202632, "support": 597336}, "weighted avg": {"f1-score": 0.9256068904244685, "precision": 0.9569692241203831, "recall": 0.9027314610202632, "support": 597336}, "\u2205": {"f1-score": 0.9788293730250339, "precision": 0.9846779872169863, "recall": 0.9730498257070407, "support": 298922}, "\u23ce": {"f1-score": 0.7872392160676943, "precision": 0.8861639523744355, "recall": 0.7081829516372465, "support": 45717}, "\u23ce\u23ce": {"f1-score": 0.6681091251175917, "precision": 0.9037923135657928, "recall": 0.5299209073272646, "support": 6701}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 207}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7234651870413461, "precision": 0.961465271170314, "recall": 0.5799139167862267, "support": 27880}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 635}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7834314761689063, "precision": 0.9186190616701092, "recall": 0.6829289694204361, "support": 22793}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2174}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 461}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 149}, "\u2423": {"f1-score": 0.9513171990650565, "precision": 0.9534703538986194, "recall": 0.9491737469558299, "support": 164659}},
  "ppcr": 0.9357179209021388
}
```
</details>
