# Test report for javascript / file:///tmp/top-repos-quality-repos-th0k9xmq/codeceptjs.git HEAD 42717c09173fdf8276166c7918b0bbd5c9f1055a

### Classification report

PPCR: 0.994

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.989| 0.997| 0.995| 0.993| 0.992| 27386| 27457| 0.997 |
| `␣` | 0.961| 0.973| 0.966| 0.967| 0.964| 9699| 9767| 0.993 |
| `'` | 0.993| 0.996| 0.996| 0.994| 0.994| 5370| 5370| 1.000 |
| `⏎` | 0.846| 0.935| 0.915| 0.889| 0.879| 2610| 2668| 0.978 |
| `⏎␣⁺␣⁺` | 0.971| 0.863| 0.854| 0.914| 0.909| 1414| 1429| 0.990 |
| `⏎␣⁻␣⁻` | 0.961| 0.898| 0.879| 0.928| 0.918| 1308| 1335| 0.980 |
| `⏎⏎` | 0.917| 0.605| 0.578| 0.729| 0.709| 925| 969| 0.955 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 42| 42| 1.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 16| 1.000 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 12| 0.917 |
| `weighted avg` | 0.972| 0.973| 0.968| 0.972| 0.969| 48781| 49065| 0.994 |
| `macro avg` | 0.664| 0.627| 0.618| 0.641| 0.636| 48781| 49065| 0.994 |
| `micro avg` | 0.973| 0.973| 0.968| 0.973| 0.971| 48781| 49065| 0.994 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|71 |27308 |75 |0 |1 |0 |0 |2 |0 |0 |0 |
|68 |170 |9437 |14 |34 |36 |6 |2 |0 |0 |0 |
|0 |0 |24 |5346 |0 |0 |0 |0 |0 |0 |0 |
|58 |27 |100 |0 |2441 |0 |0 |42 |0 |0 |0 |
|15 |90 |73 |9 |22 |1220 |0 |0 |0 |0 |0 |
|27 |5 |84 |0 |45 |0 |1174 |0 |0 |0 |0 |
|44 |5 |24 |0 |336 |0 |0 |560 |0 |0 |0 |
|0 |0 |0 |16 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |1 |0 |0 |0 |41 |0 |0 |0 |0 |
|1 |0 |0 |0 |5 |0 |1 |5 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u0027": {"f1-score": 0.9941422594142258, "precision": 0.992757660167131, "recall": 0.9955307262569832, "support": 5370}, "macro avg": {"f1-score": 0.6414070661979276, "precision": 0.6638174351639357, "recall": 0.6266677991666885, "support": 48781}, "micro avg": {"f1-score": 0.9734527787458231, "precision": 0.9734527787458231, "recall": 0.9734527787458231, "support": 48781}, "weighted avg": {"f1-score": 0.9720401911843587, "precision": 0.9723469103470473, "recall": 0.9734527787458231, "support": 48781}, "\u2205": {"f1-score": 0.9931807022967395, "precision": 0.9892410795145807, "recall": 0.9971518294018842, "support": 27386}, "\u23ce": {"f1-score": 0.888605751729159, "precision": 0.8463938973647711, "recall": 0.9352490421455939, "support": 2610}, "\u23ce\u23ce": {"f1-score": 0.7291666666666667, "precision": 0.9165302782324058, "recall": 0.6054054054054054, "support": 925}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9138576779026217, "precision": 0.9713375796178344, "recall": 0.8628005657708628, "support": 1414}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9280632411067194, "precision": 0.9607201309328969, "recall": 0.8975535168195719, "support": 1308}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u2423": {"f1-score": 0.9670543628631448, "precision": 0.9611937258097372, "recall": 0.9729869058665842, "support": 9699}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u0027": {"f1-score": 0.9941422594142258, "precision": 0.992757660167131, "recall": 0.9955307262569832, "support": 5370}, "macro avg": {"f1-score": 0.6364938449988167, "precision": 0.6638174351639357, "recall": 0.618229436015548, "support": 49065}, "micro avg": {"f1-score": 0.9706273123070949, "precision": 0.9734527787458231, "recall": 0.9678182003464791, "support": 49065}, "weighted avg": {"f1-score": 0.9689794609279694, "precision": 0.9721304312475034, "recall": 0.9678182003464791, "support": 49065}, "\u2205": {"f1-score": 0.9919000399549599, "precision": 0.9892410795145807, "recall": 0.9945733328477255, "support": 27457}, "\u23ce": {"f1-score": 0.8793227665706052, "precision": 0.8463938973647711, "recall": 0.9149175412293853, "support": 2668}, "\u23ce\u23ce": {"f1-score": 0.7088607594936709, "precision": 0.9165302782324058, "recall": 0.5779153766769866, "support": 969}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9087523277467412, "precision": 0.9713375796178344, "recall": 0.853743876836949, "support": 1429}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9182635901447007, "precision": 0.9607201309328969, "recall": 0.8794007490636704, "support": 1335}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u2423": {"f1-score": 0.9636967066632627, "precision": 0.9611937258097372, "recall": 0.9662127572437801, "support": 9767}},
  "ppcr": 0.9942117599103231
}
```
</details>
