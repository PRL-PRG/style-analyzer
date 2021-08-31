# Train report for javascript / file:///tmp/top-repos-quality-repos-z4j29f5i/fbt.git HEAD f8ea3fecfccd4fc07fee509b1dd5afc7bbe8f155

### Classification report

PPCR: 0.952

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.961| 0.991| 0.983| 0.976| 0.972| 89468| 90184| 0.992 |
| `⏎` | 0.996| 0.986| 0.928| 0.991| 0.961| 51129| 54363| 0.941 |
| `␣` | 0.949| 0.948| 0.878| 0.948| 0.912| 19390| 20940| 0.926 |
| `'` | 0.946| 0.966| 0.835| 0.956| 0.887| 7042| 8146| 0.864 |
| `⏎␣⁻␣⁻` | 0.937| 0.863| 0.846| 0.898| 0.889| 4001| 4078| 0.981 |
| `⏎␣⁺␣⁺` | 0.941| 0.679| 0.575| 0.789| 0.714| 3614| 4271| 0.846 |
| `"` | 0.896| 0.842| 0.699| 0.868| 0.786| 2454| 2954| 0.831 |
| `⏎⏎` | 0.891| 0.343| 0.173| 0.495| 0.289| 1045| 2072| 0.504 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 228| 230| 0.991 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 66| 152| 0.434 |
| `weighted avg` | 0.965| 0.967| 0.921| 0.965| 0.938| 178437| 187390| 0.952 |
| `micro avg` | 0.967| 0.967| 0.921| 0.967| 0.944| 178437| 187390| 0.952 |
| `macro avg` | 0.752| 0.662| 0.592| 0.692| 0.641| 178437| 187390| 0.952 |

### Confusion matrix

|refusal|  ∅| ⏎| ␣| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|716 |88645 |0 |681 |0 |142 |0 |0 |0 |0 |0 |
|3234 |538 |50429 |64 |0 |2 |55 |0 |41 |0 |0 |
|1550 |896 |54 |18376 |0 |4 |57 |0 |3 |0 |0 |
|1104 |0 |0 |0 |6802 |0 |0 |240 |0 |0 |0 |
|657 |912 |4 |244 |0 |2454 |0 |0 |0 |0 |0 |
|77 |424 |123 |3 |0 |0 |3451 |0 |0 |0 |0 |
|500 |0 |0 |0 |388 |0 |0 |2066 |0 |0 |0 |
|1027 |671 |16 |0 |0 |0 |0 |0 |358 |0 |0 |
|2 |106 |1 |2 |0 |0 |119 |0 |0 |0 |0 |
|86 |59 |0 |2 |0 |5 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| demo-app/src/example/Example.react.js | 370 |
| packages/babel-plugin-fbt/src/__tests__/fbtFunctional-test.js | 357 |
| website/src/pages/index.js | 154 |
| website/docusaurus.config.js | 149 |
| packages/babel-plugin-fbt/src/bin/__tests__/translate-test.js | 135 |
| packages/babel-plugin-fbt/src/babel-processors/JSXFbtProcessor.js | 127 |
| packages/babel-plugin-fbt/src/translate/TranslationBuilder.js | 123 |
| packages/babel-plugin-standalone/src/SparkMD5.js | 123 |
| packages/babel-plugin-fbt/src/bin/collectFbt.js | 115 |
| runtime/nonfb/IntlPhonologicalRewrites.js | 115 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.8680672268907563, "precision": 0.8959236773633998, "recall": 0.8418907905460473, "support": 2454}, "\u0027": {"f1-score": 0.9558740865654862, "precision": 0.9460361613351878, "recall": 0.9659187730758307, "support": 7042}, "macro avg": {"f1-score": 0.692099312333975, "precision": 0.7516666936175267, "recall": 0.6616768994565099, "support": 178437}, "micro avg": {"f1-score": 0.9671816943795288, "precision": 0.967181694379529, "recall": 0.967181694379529, "support": 178437}, "weighted avg": {"f1-score": 0.964899863046074, "precision": 0.9652481767677522, "recall": 0.967181694379529, "support": 178437}, "\u2205": {"f1-score": 0.9756272046401311, "precision": 0.960910992834766, "recall": 0.9908011803102785, "support": 89468}, "\u23ce": {"f1-score": 0.99117496756948, "precision": 0.9960890433958165, "recall": 0.9863091396272174, "support": 51129}, "\u23ce\u23ce": {"f1-score": 0.49481686247408424, "precision": 0.8905472636815921, "recall": 0.34258373205741627, "support": 1045}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7889406847773669, "precision": 0.9413118527042578, "recall": 0.6790260099612617, "support": 3614}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8983469998698425, "precision": 0.9372623574144486, "recall": 0.8625343664083979, "support": 4001}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 228}, "\u2423": {"f1-score": 0.9481450905526031, "precision": 0.948585587445798, "recall": 0.9477050025786488, "support": 19390}},
  "cl_report_full": {"\"": {"f1-score": 0.7855513307984789, "precision": 0.8959236773633998, "recall": 0.6993906567366283, "support": 2954}, "\u0027": {"f1-score": 0.8870631194574856, "precision": 0.9460361613351878, "recall": 0.8350110483672968, "support": 8146}, "macro avg": {"f1-score": 0.6409167704135694, "precision": 0.7516666936175267, "recall": 0.5916126900255142, "support": 187390}, "micro avg": {"f1-score": 0.9435115505416496, "precision": 0.967181694379529, "recall": 0.9209723037515343, "support": 187390}, "weighted avg": {"f1-score": 0.9380217264274224, "precision": 0.9643697620204095, "recall": 0.9209723037515343, "support": 187390}, "\u2205": {"f1-score": 0.9717981746923562, "precision": 0.960910992834766, "recall": 0.9829348886720483, "support": 90184}, "\u23ce": {"f1-score": 0.9606438708448423, "precision": 0.9960890433958165, "recall": 0.9276346044184464, "support": 54363}, "\u23ce\u23ce": {"f1-score": 0.28940986257073564, "precision": 0.8905472636815921, "recall": 0.17277992277992277, "support": 2072}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7135795289328293, "precision": 0.9413118527042578, "recall": 0.5745726996019668, "support": 4271}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 152}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8894329896907216, "precision": 0.9372623574144486, "recall": 0.8462481608631682, "support": 4078}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 230}, "\u2423": {"f1-score": 0.9116888271482438, "precision": 0.948585587445798, "recall": 0.8775549188156638, "support": 20940}},
  "ppcr": 0.9522226372805379
}
```
</details>
