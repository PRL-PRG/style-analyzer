# Train report for javascript / file:///tmp/top-repos-quality-repos-0pxfcpy1/crucible.git HEAD d503264b38931d00b234fe26882541d8c728b2e1

### Classification report

PPCR: 0.809

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.982| 0.995| 0.870| 0.988| 0.922| 58429| 66857| 0.874 |
| `␣` | 0.957| 0.950| 0.708| 0.954| 0.814| 19778| 26515| 0.746 |
| `'` | 0.994| 1.000| 0.931| 0.997| 0.961| 6460| 6940| 0.931 |
| `⏎␣⁺␣⁺` | 0.982| 0.894| 0.824| 0.936| 0.896| 5297| 5751| 0.921 |
| `⏎` | 0.968| 0.977| 0.584| 0.973| 0.728| 5073| 8497| 0.597 |
| `⏎␣⁻␣⁻` | 0.982| 0.987| 0.831| 0.984| 0.900| 4689| 5570| 0.842 |
| `⏎⏎` | 0.924| 0.764| 0.262| 0.837| 0.408| 526| 1535| 0.343 |
| `␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 51| 362| 0.141 |
| `"` | 1.000| 0.045| 0.020| 0.087| 0.038| 44| 102| 0.431 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 103| 0.243 |
| `␣␣␣␣␣␣␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 193| 0.119 |
| `␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 326| 0.043 |
| `␣␣␣␣␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 219| 0.032 |
| `␣␣␣␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 233| 0.017 |
| `␣␣␣␣␣␣␣␣␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 130| 0.031 |
| `␣␣␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 264| 0.011 |
| `␣␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 253| 0.008 |
| `␣␣␣␣␣␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 189| 0.005 |
| `␣␣␣␣␣␣␣␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 132| 0.000 |
| `micro avg` | 0.977| 0.977| 0.790| 0.977| 0.874| 100430| 124171| 0.809 |
| `weighted avg` | 0.975| 0.977| 0.790| 0.976| 0.861| 100430| 124171| 0.809 |
| `macro avg` | 0.410| 0.348| 0.265| 0.356| 0.298| 100430| 124171| 0.809 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ␣␣| ␣␣␣| ␣␣␣␣␣| ␣␣␣␣| ␣␣␣␣␣␣| ␣␣␣␣␣␣␣| ␣␣␣␣␣␣␣␣| ␣␣␣␣␣␣␣␣␣| ␣␣␣␣␣␣␣␣␣␣| ␣␣␣␣␣␣␣␣␣␣␣| ⏎␣⁻␣⁻␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8428 |58133 |248 |23 |0 |12 |13 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6737 |889 |18784 |17 |0 |50 |38 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3424 |31 |49 |4958 |0 |1 |1 |33 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|480 |0 |0 |0 |6460 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|454 |124 |437 |0 |0 |4736 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|881 |8 |9 |43 |0 |0 |4629 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1009 |2 |8 |79 |0 |22 |13 |402 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|311 |9 |42 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|312 |5 |9 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|261 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|251 |1 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|229 |3 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|212 |3 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|188 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|170 |1 |22 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|132 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|126 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|78 |1 |0 |2 |0 |0 |22 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|58 |0 |0 |0 |42 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |2 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/lib/semantic-ui/definitions/modules/dropdown.js | 316 |
| client/lib/semantic-ui/definitions/behaviors/form.js | 176 |
| client/lib/semantic-ui/definitions/modules/popup.js | 162 |
| client/lib/semantic-ui/definitions/modules/search.js | 124 |
| client/lib/semantic-ui/definitions/modules/shape.js | 114 |
| client/lib/semantic-ui/definitions/behaviors/visibility.js | 104 |
| client/lib/semantic-ui/definitions/behaviors/api.js | 101 |
| client/lib/semantic-ui/definitions/behaviors/state.js | 92 |
| client/lib/semantic-ui/definitions/modules/sticky.js | 90 |
| client/lib/semantic-ui/definitions/modules/transition.js | 89 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.08695652173913045, "precision": 1.0, "recall": 0.045454545454545456, "support": 44}, "\u0027": {"f1-score": 0.9967597592964048, "precision": 0.9935404490925869, "recall": 1.0, "support": 6460}, "macro avg": {"f1-score": 0.35554717311899936, "precision": 0.40993751614606755, "recall": 0.34805343780351755, "support": 100430}, "micro avg": {"f1-score": 0.9768395897640147, "precision": 0.9768395897640147, "recall": 0.9768395897640147, "support": 100430}, "weighted avg": {"f1-score": 0.9757744582236131, "precision": 0.9754512726634137, "recall": 0.9768395897640147, "support": 100430}, "\u2205": {"f1-score": 0.9882950961808182, "precision": 0.9817441821190934, "recall": 0.9949340224888326, "support": 58429}, "\u23ce": {"f1-score": 0.9726336439431095, "precision": 0.9679812573213589, "recall": 0.977330967869111, "support": 5073}, "\u23ce\u23ce": {"f1-score": 0.8366285119667013, "precision": 0.9241379310344827, "recall": 0.7642585551330798, "support": 526}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9361533899980234, "precision": 0.9823688031528729, "recall": 0.8940909949027751, "support": 5297}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9843700159489632, "precision": 0.9815521628498728, "recall": 0.9872040946896993, "support": 4689}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u2423": {"f1-score": 0.9535993501878363, "precision": 0.9574880212050157, "recall": 0.9497421377287896, "support": 19778}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 51}, "\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423\u2423\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}},
  "cl_report_full": {"\"": {"f1-score": 0.038461538461538464, "precision": 1.0, "recall": 0.0196078431372549, "support": 102}, "\u0027": {"f1-score": 0.9611664930813867, "precision": 0.9935404490925869, "recall": 0.930835734870317, "support": 6940}, "macro avg": {"f1-score": 0.2983383009130768, "precision": 0.40993751614606755, "recall": 0.26464962217413013, "support": 124171}, "micro avg": {"f1-score": 0.8735847124456257, "precision": 0.9768395897640147, "recall": 0.7900717558850295, "support": 124171}, "weighted avg": {"f1-score": 0.8609340419658496, "precision": 0.9565983473153413, "recall": 0.7900717558850295, "support": 124171}, "\u2205": {"f1-score": 0.9222263645088878, "precision": 0.9817441821190934, "recall": 0.8695125416934651, "support": 66857}, "\u23ce": {"f1-score": 0.728100447903664, "precision": 0.9679812573213589, "recall": 0.583500058844298, "support": 8497}, "\u23ce\u23ce": {"f1-score": 0.40812182741116754, "precision": 0.9241379310344827, "recall": 0.26188925081433223, "support": 1535}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8959515701853954, "precision": 0.9823688031528729, "recall": 0.8235089549643541, "support": 5751}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9000583317130081, "precision": 0.9815521628498728, "recall": 0.8310592459605027, "support": 5570}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 103}, "\u2423": {"f1-score": 0.814341144083411, "precision": 0.9574880212050157, "recall": 0.7084291910239487, "support": 26515}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 362}, "\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 326}, "\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 253}, "\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 264}, "\u2423\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 233}, "\u2423\u2423\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 219}, "\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 189}, "\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 193}, "\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 132}, "\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}},
  "ppcr": 0.8088039880487392
}
```
</details>