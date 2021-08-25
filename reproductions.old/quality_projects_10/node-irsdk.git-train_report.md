# Train report for javascript / file:///tmp/top-repos-quality-repos-gqm2jq0w/node-irsdk.git HEAD 986a9e079cb8b8b780f8d55f1f0988c85596857a

### Classification report

PPCR: 0.528

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.987| 0.999| 0.838| 0.993| 0.906| 3471| 4136| 0.839 |
| `'` | 1.000| 1.000| 0.768| 1.000| 0.869| 460| 599| 0.768 |
| `⏎␣⁻␣⁻` | 0.936| 1.000| 0.896| 0.967| 0.916| 206| 230| 0.896 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 55| 1997| 0.028 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 618| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 234| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 121| 0.000 |
| `macro avg` | 0.418| 0.428| 0.357| 0.423| 0.384| 4192| 7935| 0.528 |
| `micro avg` | 0.986| 0.986| 0.521| 0.986| 0.681| 4192| 7935| 0.528 |
| `weighted avg` | 0.973| 0.986| 0.521| 0.979| 0.565| 4192| 7935| 0.528 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|665 |3466 |0 |0 |0 |0 |5 |0 |
|1942 |46 |0 |0 |0 |0 |9 |0 |
|139 |0 |0 |460 |0 |0 |0 |0 |
|618 |0 |0 |0 |0 |0 |0 |0 |
|234 |0 |0 |0 |0 |0 |0 |0 |
|24 |0 |0 |0 |0 |0 |206 |0 |
|121 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/JsIrSdk.js | 25 |
| test/smoke-test.js | 8 |
| src/IrsdkNodeWrapper-stub.js | 7 |
| utils/sample-writer.js | 7 |
| utils/recorder.js | 5 |
| src/JsIrSdk-spec.js | 2 |
| utils/print-names.js | 2 |
| utils/cam-example.js | 2 |
| src/node-irsdk.js | 1 |
| src/node-irsdk-spec.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 460}, "macro avg": {"f1-score": 0.422833242713717, "precision": 0.4176093837825045, "recall": 0.4283656418487879, "support": 4192}, "micro avg": {"f1-score": 0.9856870229007635, "precision": 0.9856870229007634, "recall": 0.9856870229007634, "support": 4192}, "weighted avg": {"f1-score": 0.979217501836534, "precision": 0.9729074248655151, "recall": 0.9856870229007634, "support": 4192}, "\u2205": {"f1-score": 0.9926965487612773, "precision": 0.9869020501138952, "recall": 0.9985594929415154, "support": 3471}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9671361502347418, "precision": 0.9363636363636364, "recall": 1.0, "support": 206}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8687440982058546, "precision": 1.0, "recall": 0.7679465776293823, "support": 599}, "macro avg": {"f1-score": 0.3843829152713551, "precision": 0.4176093837825045, "recall": 0.3573723554980476, "support": 7935}, "micro avg": {"f1-score": 0.6814546054259091, "precision": 0.9856870229007634, "recall": 0.5207309388783868, "support": 7935}, "weighted avg": {"f1-score": 0.5645552977419236, "precision": 0.6170372420459618, "recall": 0.5207309388783868, "support": 7935}, "\u2205": {"f1-score": 0.9063807531380753, "precision": 0.9869020501138952, "recall": 0.8380077369439072, "support": 4136}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 618}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 234}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9155555555555557, "precision": 0.9363636363636364, "recall": 0.8956521739130435, "support": 230}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1997}},
  "ppcr": 0.5282923755513548
}
```
</details>
