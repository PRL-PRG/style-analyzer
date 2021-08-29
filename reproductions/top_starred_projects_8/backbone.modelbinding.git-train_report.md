# Train report for javascript / file:///tmp/top-repos-quality-repos-1fxdhitm/backbone.modelbinding.git HEAD ca30d8154bd31c85ccb7134ce9fe55daa5a579ee

### Classification report

PPCR: 0.836

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 0.995| 0.961| 0.989| 0.972| 21936| 22690| 0.967 |
| `␣` | 0.976| 0.950| 0.777| 0.963| 0.865| 7543| 9222| 0.818 |
| `⏎␣⁻␣⁻` | 0.970| 0.982| 0.966| 0.976| 0.968| 1168| 1187| 0.984 |
| `⏎␣⁺␣⁺` | 0.982| 0.983| 0.921| 0.983| 0.951| 1133| 1209| 0.937 |
| `"` | 1.000| 1.000| 0.447| 1.000| 0.618| 699| 1563| 0.447 |
| `⏎` | 0.960| 0.884| 0.237| 0.921| 0.381| 518| 1929| 0.269 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 918| 0.028 |
| `'` | 1.000| 1.000| 0.002| 1.000| 0.005| 2| 801| 0.002 |
| `weighted avg` | 0.980| 0.981| 0.820| 0.981| 0.861| 33025| 39519| 0.836 |
| `macro avg` | 0.859| 0.849| 0.539| 0.854| 0.595| 33025| 39519| 0.836 |
| `micro avg` | 0.981| 0.981| 0.820| 0.981| 0.893| 33025| 39519| 0.836 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|754 |21816 |120 |0 |0 |0 |0 |0 |0 |
|1679 |325 |7166 |0 |0 |17 |35 |0 |0 |
|1411 |30 |29 |458 |0 |1 |0 |0 |0 |
|864 |0 |0 |0 |699 |0 |0 |0 |0 |
|76 |1 |17 |1 |0 |1114 |0 |0 |0 |
|19 |13 |7 |0 |0 |1 |1147 |0 |0 |
|799 |0 |0 |0 |0 |0 |0 |2 |0 |
|892 |6 |0 |18 |0 |1 |1 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| public/javascripts/backbone.js | 178 |
| spec/javascripts/support/jasmine-standalone/jasmine.js | 154 |
| public/javascripts/underscore.js | 116 |
| backbone.modelbinding.js | 93 |
| spec/javascripts/support/jasmine-standalone/jasmine-html.js | 36 |
| spec/javascripts/helpers/jasmine-jquery.js | 14 |
| spec/javascripts/helpers/sample.backbone.app.js | 7 |
| spec/javascripts/globalConfigurableBindingAttributes.spec.js | 4 |
| spec/javascripts/customConvention.spec.js | 3 |
| spec/javascripts/helpers/SpecHelper.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 699}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2}, "macro avg": {"f1-score": 0.8538724550546402, "precision": 0.8589535608332655, "recall": 0.8492462775568296, "support": 33025}, "micro avg": {"f1-score": 0.9811355034065102, "precision": 0.9811355034065102, "recall": 0.9811355034065102, "support": 33025}, "weighted avg": {"f1-score": 0.9806275062277845, "precision": 0.9802979963500721, "recall": 0.9811355034065102, "support": 33025}, "\u2205": {"f1-score": 0.9887823781358352, "precision": 0.9831012572664594, "recall": 0.9945295404814004, "support": 21936}, "\u23ce": {"f1-score": 0.920603015075377, "precision": 0.960167714884696, "recall": 0.8841698841698842, "support": 518}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9827966475518306, "precision": 0.982363315696649, "recall": 0.9832303618711385, "support": 1133}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9757549978732453, "precision": 0.9695688926458157, "recall": 0.9820205479452054, "support": 1168}, "\u2423": {"f1-score": 0.9630426018008332, "precision": 0.9764273061725031, "recall": 0.9500198859870078, "support": 7543}},
  "cl_report_full": {"\"": {"f1-score": 0.6180371352785146, "precision": 1.0, "recall": 0.4472168905950096, "support": 1563}, "\u0027": {"f1-score": 0.0049813200498132005, "precision": 1.0, "recall": 0.0024968789013732834, "support": 801}, "macro avg": {"f1-score": 0.5950201226713201, "precision": 0.8589535608332655, "recall": 0.5391753063028948, "support": 39519}, "micro avg": {"f1-score": 0.8933061314512573, "precision": 0.9811355034065102, "recall": 0.8199094106632253, "support": 39519}, "weighted avg": {"f1-score": 0.861416366139535, "precision": 0.9581694676271969, "recall": 0.8199094106632253, "support": 39519}, "\u2205": {"f1-score": 0.9721708518081148, "precision": 0.9831012572664594, "recall": 0.9614808285588365, "support": 22690}, "\u23ce": {"f1-score": 0.38071487946799665, "precision": 0.960167714884696, "recall": 0.23742871954380507, "support": 1929}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 918}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.950917626973965, "precision": 0.982363315696649, "recall": 0.9214226633581473, "support": 1209}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9679324894514767, "precision": 0.9695688926458157, "recall": 0.966301600673968, "support": 1187}, "\u2423": {"f1-score": 0.8654066783406799, "precision": 0.9764273061725031, "recall": 0.7770548687920191, "support": 9222}},
  "ppcr": 0.8356739796047471
}
```
</details>
