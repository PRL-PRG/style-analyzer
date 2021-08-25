# Train report for javascript / file:///tmp/top-repos-quality-repos-wicx81se/advanceonreactnative.git HEAD 4abee4713cfc3fbb3760e1c9536afd4d3dccf9ee

### Classification report

PPCR: 0.871

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.970| 0.994| 0.959| 0.982| 0.965| 285571| 295962| 0.965 |
| `␣` | 0.962| 0.978| 0.906| 0.970| 0.933| 129943| 140229| 0.927 |
| `'` | 0.951| 0.995| 0.951| 0.972| 0.951| 22348| 23383| 0.956 |
| `⏎` | 0.946| 0.883| 0.406| 0.913| 0.568| 19173| 41703| 0.460 |
| `⏎␣⁺␣⁺` | 0.956| 0.669| 0.405| 0.787| 0.568| 14221| 23505| 0.605 |
| `⏎␣⁻␣⁻` | 0.947| 0.768| 0.350| 0.848| 0.511| 9384| 20612| 0.455 |
| `"` | 0.970| 0.763| 0.668| 0.854| 0.791| 4843| 5532| 0.875 |
| `⏎⏎` | 0.909| 0.779| 0.341| 0.839| 0.496| 3983| 9103| 0.438 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 583| 1881| 0.310 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 577| 1126| 0.512 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 112| 353| 0.317 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 54| 124| 0.435 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 160| 0.125 |
| `micro avg` | 0.965| 0.965| 0.840| 0.965| 0.898| 490812| 563673| 0.871 |
| `weighted avg` | 0.962| 0.965| 0.840| 0.962| 0.878| 490812| 563673| 0.871 |
| `macro avg` | 0.585| 0.525| 0.383| 0.551| 0.445| 490812| 563673| 0.871 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|10391 |283894 |1494 |9 |0 |96 |75 |3 |0 |0 |0 |0 |0 |0 |
|10286 |2336 |127055 |524 |0 |2 |22 |4 |0 |0 |0 |0 |0 |0 |
|22530 |781 |1157 |16936 |0 |1 |29 |269 |0 |0 |0 |0 |0 |0 |
|1035 |0 |0 |0 |22232 |0 |0 |0 |116 |0 |0 |0 |0 |0 |
|9284 |3342 |1342 |28 |0 |9509 |0 |0 |0 |0 |0 |0 |0 |0 |
|11228 |1731 |342 |85 |0 |0 |7204 |22 |0 |0 |0 |0 |0 |0 |
|5120 |40 |545 |296 |0 |0 |0 |3102 |0 |0 |0 |0 |0 |0 |
|689 |0 |0 |0 |1148 |0 |0 |0 |3695 |0 |0 |0 |0 |0 |
|1298 |270 |18 |23 |0 |0 |272 |0 |0 |0 |0 |0 |0 |0 |
|549 |125 |109 |2 |0 |341 |0 |0 |0 |0 |0 |0 |0 |0 |
|241 |101 |1 |5 |0 |0 |5 |0 |0 |0 |0 |0 |0 |0 |
|140 |0 |5 |2 |0 |0 |0 |13 |0 |0 |0 |0 |0 |0 |
|70 |52 |0 |0 |0 |0 |2 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| react-native-0.59-stable/Libraries/Renderer/oss/ReactNativeRenderer-profiling.js | 1450 |
| react-native-0.59-stable/Libraries/Renderer/oss/ReactNativeRenderer-prod.js | 1418 |
| react-native-0.59-stable/Libraries/Renderer/oss/ReactFabric-profiling.js | 1404 |
| react-native-0.59-stable/Libraries/Renderer/oss/ReactFabric-prod.js | 1372 |
| AwesomeProject/DrawFiber/FiberTreeTab.2.1.js | 353 |
| react-native-0.59-stable/Libraries/Lists/VirtualizedList.js | 251 |
| react-native-0.59-stable/Libraries/Components/TextInput/TextInput.js | 190 |
| AwesomeProject/DrawFiber/FiberTree.2.0.js | 174 |
| AwesomeProject/PseudoCode.js | 169 |
| react-native-0.59-stable/RNTester/js/TextInputExample.ios.js | 161 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.853940374393344, "precision": 0.9695617948045132, "recall": 0.762956844930828, "support": 4843}, "\u0027": {"f1-score": 0.9723582925122463, "precision": 0.9508982035928144, "recall": 0.9948093789153392, "support": 22348}, "macro avg": {"f1-score": 0.5511559077964617, "precision": 0.5853501893155205, "recall": 0.5252425239519789, "support": 490812}, "micro avg": {"f1-score": 0.9649865936448172, "precision": 0.9649865936448172, "recall": 0.9649865936448172, "support": 490812}, "weighted avg": {"f1-score": 0.9622792568260204, "precision": 0.9620585166628658, "recall": 0.9649865936448172, "support": 490812}, "\u2205": {"f1-score": 0.9819193660796586, "precision": 0.9700073802755302, "recall": 0.9941275549688169, "support": 285571}, "\u23ce": {"f1-score": 0.913410457622091, "precision": 0.9456169737576773, "recall": 0.8833255098315339, "support": 19173}, "\u23ce\u23ce": {"f1-score": 0.8388318009734993, "precision": 0.9088778200996191, "recall": 0.778809942254582, "support": 3983}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.786843194042201, "precision": 0.9557744496934365, "recall": 0.668659025384994, "support": 14221}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 577}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8478785382216207, "precision": 0.9467735576291234, "recall": 0.76768968456948, "support": 9384}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 583}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}, "\u2423": {"f1-score": 0.9698447775093412, "precision": 0.9620422812490536, "recall": 0.9777748705201511, "support": 129943}},
  "cl_report_full": {"\"": {"f1-score": 0.790966498983196, "precision": 0.9695617948045132, "recall": 0.6679320318148951, "support": 5532}, "\u0027": {"f1-score": 0.9508372003507046, "precision": 0.9508982035928144, "recall": 0.9507762049352093, "support": 23383}, "macro avg": {"f1-score": 0.44480816051349564, "precision": 0.5853501893155205, "recall": 0.3834554168524778, "support": 563673}, "micro avg": {"f1-score": 0.8983096013693889, "precision": 0.9649865936448172, "recall": 0.8402513514040942, "support": 563673}, "weighted avg": {"f1-score": 0.8782503511642256, "precision": 0.9567230900623735, "recall": 0.8402513514040942, "support": 563673}, "\u2205": {"f1-score": 0.9645858037422236, "precision": 0.9700073802755302, "recall": 0.959224495036525, "support": 295962}, "\u23ce": {"f1-score": 0.5681982117994397, "precision": 0.9456169737576773, "recall": 0.40610987219144906, "support": 41703}, "\u23ce\u23ce": {"f1-score": 0.4956855225311601, "precision": 0.9088778200996191, "recall": 0.34076678018235745, "support": 9103}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 160}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5684820948167634, "precision": 0.9557744496934365, "recall": 0.4045522229312912, "support": 23505}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1126}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5105417951171113, "precision": 0.9467735576291234, "recall": 0.34950514263535803, "support": 20612}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1881}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 353}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 124}, "\u2423": {"f1-score": 0.9332089593348439, "precision": 0.9620422812490536, "recall": 0.9060536693551262, "support": 140229}},
  "ppcr": 0.8707388858433879
}
```
</details>
