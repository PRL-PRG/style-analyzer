# Train report for javascript / file:///tmp/top-repos-quality-repos-kf6x7s6e/kidprojects.git HEAD ecb7937431803842f4097c7782bcf87c69a66f82

### Classification report

PPCR: 0.702

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.986| 0.880| 0.988| 0.932| 24874| 27881| 0.892 |
| `␣` | 0.955| 0.983| 0.575| 0.969| 0.718| 15082| 25761| 0.585 |
| `"` | 0.963| 1.000| 0.889| 0.981| 0.924| 2747| 3090| 0.889 |
| `⏎` | 0.974| 0.868| 0.450| 0.918| 0.616| 2073| 4000| 0.518 |
| `⏎⇥⁺` | 0.921| 0.916| 0.657| 0.919| 0.767| 1064| 1483| 0.717 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.923| 0.925| 0.836| 0.924| 0.878| 856| 947| 0.904 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.986| 0.877| 0.148| 0.928| 0.258| 155| 918| 0.169 |
| `'` | 1.000| 0.019| 0.010| 0.036| 0.020| 108| 200| 0.540 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 68| 1052| 0.065 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 189| 0.069 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 1348| 0.006 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 203| 0.020 |
| `macro avg` | 0.643| 0.548| 0.371| 0.555| 0.426| 47052| 67072| 0.702 |
| `micro avg` | 0.974| 0.974| 0.683| 0.974| 0.803| 47052| 67072| 0.702 |
| `weighted avg` | 0.972| 0.974| 0.683| 0.971| 0.775| 47052| 67072| 0.702 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎⏎⇥⁻| ⏎⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3007 |24529 |295 |30 |0 |12 |0 |0 |8 |0 |0 |0 |0 |
|10679 |230 |14825 |5 |0 |9 |0 |0 |11 |2 |0 |0 |0 |
|1927 |5 |237 |1800 |0 |7 |0 |0 |24 |0 |0 |0 |0 |
|343 |0 |0 |0 |2747 |0 |0 |0 |0 |0 |0 |0 |0 |
|419 |2 |64 |1 |0 |975 |0 |0 |22 |0 |0 |0 |0 |
|1340 |0 |6 |0 |0 |2 |0 |0 |0 |0 |0 |0 |0 |
|984 |0 |31 |12 |0 |25 |0 |0 |0 |0 |0 |0 |0 |
|91 |4 |49 |0 |0 |11 |0 |0 |792 |0 |0 |0 |0 |
|763 |3 |13 |0 |0 |2 |0 |0 |1 |136 |0 |0 |0 |
|92 |0 |0 |0 |106 |0 |0 |0 |0 |0 |2 |0 |0 |
|199 |0 |1 |0 |0 |3 |0 |0 |0 |0 |0 |0 |0 |
|176 |0 |0 |0 |0 |13 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| dotnet/storProcedure CRUD/stor/Scripts/jquery-3.3.1.slim.js | 564 |
| dotnet/storProcedure CRUD/stor/Scripts/WebForms/SmartNav.js | 115 |
| dotnet/storProcedure CRUD/stor/Scripts/WebForms/WebUIValidation.js | 91 |
| js/ReactjsBasic/src/App.js | 84 |
| js/nodejs CRUD/controller/employeeControl.js | 74 |
| js/File upload/server.js | 61 |
| dotnet/storProcedure CRUD/stor/Scripts/WebForms/WebForms.js | 38 |
| dotnet/storProcedure CRUD/stor/Scripts/WebForms/Menu.js | 36 |
| js/ReactjsBasic/src/Person/Person.js | 35 |
| js/ReactjsBasic/src/UserInput/userInput.js | 24 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9810714285714285, "precision": 0.9628461268839817, "recall": 1.0, "support": 2747}, "\u0027": {"f1-score": 0.03636363636363636, "precision": 1.0, "recall": 0.018518518518518517, "support": 108}, "macro avg": {"f1-score": 0.5552964091054647, "precision": 0.6426203543923936, "recall": 0.5479101348955921, "support": 47052}, "micro avg": {"f1-score": 0.9735186602057299, "precision": 0.9735186602057299, "recall": 0.9735186602057299, "support": 47052}, "weighted avg": {"f1-score": 0.9713873936503645, "precision": 0.9718884654371368, "recall": 0.9735186602057299, "support": 47052}, "\u2205": {"f1-score": 0.9881362418675852, "precision": 0.9901505671497195, "recall": 0.9861300956822385, "support": 24874}, "\u23ce": {"f1-score": 0.918133129303749, "precision": 0.974025974025974, "recall": 0.8683068017366136, "support": 2073}, "\u23ce\u21e5\u207a": {"f1-score": 0.9185115402731984, "precision": 0.9206798866855525, "recall": 0.9163533834586466, "support": 1064}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9241540256709452, "precision": 0.9230769230769231, "recall": 0.9252336448598131, "support": 856}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9283276450511947, "precision": 0.9855072463768116, "recall": 0.8774193548387097, "support": 155}, "\u2423": {"f1-score": 0.9688592621638401, "precision": 0.955157528509761, "recall": 0.982959819652566, "support": 15082}},
  "cl_report_full": {"\"": {"f1-score": 0.9244489315160693, "precision": 0.9628461268839817, "recall": 0.8889967637540453, "support": 3090}, "\u0027": {"f1-score": 0.019801980198019802, "precision": 1.0, "recall": 0.01, "support": 200}, "macro avg": {"f1-score": 0.4260026983950495, "precision": 0.6426203543923936, "recall": 0.37051486144501083, "support": 67072}, "micro avg": {"f1-score": 0.8027408783428552, "precision": 0.9735186602057299, "recall": 0.6829377385496184, "support": 67072}, "weighted avg": {"f1-score": 0.7753940950926875, "precision": 0.9307567577096123, "recall": 0.6829377385496184, "support": 67072}, "\u2205": {"f1-score": 0.9317050936301136, "precision": 0.9901505671497195, "recall": 0.8797747570029769, "support": 27881}, "\u23ce": {"f1-score": 0.6155950752393982, "precision": 0.974025974025974, "recall": 0.45, "support": 4000}, "\u23ce\u21e5\u207a": {"f1-score": 0.7671125098347759, "precision": 0.9206798866855525, "recall": 0.6574511126095752, "support": 1483}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1348}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1052}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 189}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 203}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8775623268698062, "precision": 0.9230769230769231, "recall": 0.8363252375923971, "support": 947}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.25757575757575757, "precision": 0.9855072463768116, "recall": 0.14814814814814814, "support": 918}, "\u2423": {"f1-score": 0.7182307058766533, "precision": 0.955157528509761, "recall": 0.5754823182329879, "support": 25761}},
  "ppcr": 0.7015147900763359
}
```
</details>
