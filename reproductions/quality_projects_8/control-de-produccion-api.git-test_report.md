# Test report for javascript / file:///tmp/top-repos-quality-repos-ud6ubt_1/control-de-produccion-api.git HEAD 0988c2224a2dc61e5c7e83a45480a7c516772db0

### Classification report

PPCR: 0.949

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.947| 0.979| 0.974| 0.963| 0.960| 6212| 6244| 0.995 |
| `␣` | 0.789| 0.965| 0.917| 0.868| 0.848| 3155| 3321| 0.950 |
| `"` | 0.911| 0.863| 0.863| 0.887| 0.887| 2078| 2078| 1.000 |
| `⏎` | 0.800| 0.600| 0.495| 0.686| 0.612| 1021| 1238| 0.825 |
| `⏎␣⁺␣⁺` | 0.783| 0.605| 0.547| 0.683| 0.644| 466| 516| 0.903 |
| `⏎␣⁻␣⁻` | 0.856| 0.569| 0.444| 0.684| 0.585| 325| 417| 0.779 |
| `⏎⏎` | 0.846| 0.311| 0.183| 0.455| 0.301| 177| 300| 0.590 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 164| 164| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.911| 0.719| 0.406| 0.804| 0.562| 57| 101| 0.564 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 61| 0.770 |
| `micro avg` | 0.883| 0.883| 0.838| 0.883| 0.859| 13702| 14440| 0.949 |
| `weighted avg` | 0.870| 0.883| 0.838| 0.870| 0.840| 13702| 14440| 0.949 |
| `macro avg` | 0.684| 0.561| 0.483| 0.603| 0.540| 13702| 14440| 0.949 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| '| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|32 |6080 |83 |0 |28 |12 |5 |4 |0 |0 |0 |
|166 |55 |3044 |0 |15 |27 |14 |0 |0 |0 |0 |
|0 |28 |206 |1794 |50 |0 |0 |0 |0 |0 |0 |
|217 |65 |336 |1 |613 |0 |0 |6 |0 |0 |0 |
|50 |36 |118 |11 |19 |282 |0 |0 |0 |0 |0 |
|92 |94 |41 |0 |1 |0 |185 |0 |0 |4 |0 |
|123 |57 |25 |0 |40 |0 |0 |55 |0 |0 |0 |
|0 |1 |0 |163 |0 |0 |0 |0 |0 |0 |0 |
|44 |3 |1 |0 |0 |0 |12 |0 |0 |41 |0 |
|14 |2 |6 |0 |0 |39 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.8865826538176427, "precision": 0.9111223971559167, "recall": 0.8633301251203079, "support": 2078}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 164}, "macro avg": {"f1-score": 0.6028256071426004, "precision": 0.6843957309430595, "recall": 0.5611704145103573, "support": 13702}, "micro avg": {"f1-score": 0.8826448693621369, "precision": 0.8826448693621369, "recall": 0.8826448693621369, "support": 13702}, "weighted avg": {"f1-score": 0.8704588076160947, "precision": 0.8703551985653236, "recall": 0.8826448693621369, "support": 13702}, "\u2205": {"f1-score": 0.962558378849046, "precision": 0.9468930073197321, "recall": 0.978750804893754, "support": 6212}, "\u23ce": {"f1-score": 0.6860660324566312, "precision": 0.8002610966057441, "recall": 0.6003917727717923, "support": 1021}, "\u23ce\u23ce": {"f1-score": 0.45454545454545453, "precision": 0.8461538461538461, "recall": 0.3107344632768362, "support": 177}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6828087167070219, "precision": 0.7833333333333333, "recall": 0.6051502145922747, "support": 466}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6839186691312383, "precision": 0.8564814814814815, "recall": 0.5692307692307692, "support": 325}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.803921568627451, "precision": 0.9111111111111111, "recall": 0.7192982456140351, "support": 57}, "\u2423": {"f1-score": 0.8678545972915181, "precision": 0.78860103626943, "recall": 0.9648177496038035, "support": 3155}},
  "cl_report_full": {"\"": {"f1-score": 0.8865826538176427, "precision": 0.9111223971559167, "recall": 0.8633301251203079, "support": 2078}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 164}, "macro avg": {"f1-score": 0.5397645702380623, "precision": 0.6843957309430595, "recall": 0.4828240411227063, "support": 14440}, "micro avg": {"f1-score": 0.8594982588302182, "precision": 0.8826448693621369, "recall": 0.8375346260387811, "support": 14440}, "weighted avg": {"f1-score": 0.8402594806078505, "precision": 0.8672161157818037, "recall": 0.8375346260387811, "support": 14440}, "\u2205": {"f1-score": 0.9601263324121595, "precision": 0.9468930073197321, "recall": 0.9737347853939782, "support": 6244}, "\u23ce": {"f1-score": 0.6117764471057884, "precision": 0.8002610966057441, "recall": 0.49515347334410337, "support": 1238}, "\u23ce\u23ce": {"f1-score": 0.3013698630136986, "precision": 0.8461538461538461, "recall": 0.18333333333333332, "support": 300}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6438356164383561, "precision": 0.7833333333333333, "recall": 0.5465116279069767, "support": 516}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5845181674565562, "precision": 0.8564814814814815, "recall": 0.44364508393285373, "support": 417}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.5616438356164384, "precision": 0.9111111111111111, "recall": 0.40594059405940597, "support": 101}, "\u2423": {"f1-score": 0.8477927865199834, "precision": 0.78860103626943, "recall": 0.9165913881361036, "support": 3321}},
  "ppcr": 0.9488919667590028
}
```
</details>
