# Train report for javascript / file:///tmp/top-repos-quality-repos-mocrg9z8/lbry-desktop.git HEAD 68d13f9ff31bb94afb75d52d593ae6a89e1672a6

### Classification report

PPCR: 0.932

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.968| 0.986| 0.963| 0.976| 0.965| 75658| 77459| 0.977 |
| `␣` | 0.954| 0.971| 0.928| 0.962| 0.941| 49229| 51503| 0.956 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 10038| 10038| 1.000 |
| `⏎` | 0.955| 0.828| 0.483| 0.887| 0.642| 6226| 10665| 0.584 |
| `⏎␣⁺␣⁺` | 0.907| 0.825| 0.743| 0.864| 0.817| 5153| 5717| 0.901 |
| `⏎␣⁻␣⁻` | 0.937| 0.821| 0.757| 0.875| 0.837| 4708| 5105| 0.922 |
| `⏎⏎` | 0.889| 0.672| 0.190| 0.766| 0.313| 574| 2033| 0.282 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 183| 217| 0.843 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 136| 0.235 |
| `weighted avg` | 0.960| 0.962| 0.897| 0.961| 0.919| 151801| 162873| 0.932 |
| `micro avg` | 0.962| 0.962| 0.897| 0.962| 0.928| 151801| 162873| 0.932 |
| `macro avg` | 0.734| 0.678| 0.563| 0.703| 0.613| 151801| 162873| 0.932 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1801 |74561 |1029 |17 |0 |15 |35 |1 |0 |0 |
|2274 |807 |47777 |148 |0 |413 |84 |0 |0 |0 |
|4439 |432 |608 |5153 |0 |7 |2 |24 |0 |0 |
|0 |0 |0 |0 |10038 |0 |0 |0 |0 |0 |
|564 |495 |408 |0 |0 |4250 |0 |0 |0 |0 |
|397 |665 |131 |47 |0 |0 |3865 |0 |0 |0 |
|1459 |53 |105 |30 |0 |0 |0 |386 |0 |0 |
|34 |40 |2 |2 |0 |0 |139 |0 |0 |0 |
|104 |6 |2 |0 |0 |0 |1 |23 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/webworkers/worker-bundle.js | 1275 |
| static/webworkers/wasm-gen/libarchive.js | 945 |
| ui/redux/actions/user.js | 228 |
| ui/redux/reducers/subscriptions.js | 109 |
| ui/redux/selectors/subscriptions.js | 97 |
| ui/redux/reducers/settings.js | 96 |
| web/src/html.js | 95 |
| ui/redux/reducers/search.js | 91 |
| ui/redux/reducers/comments.js | 71 |
| ui/redux/selectors/search.js | 70 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 10038}, "macro avg": {"f1-score": 0.7033790141239031, "precision": 0.7344470651011624, "recall": 0.6779825730082748, "support": 151801}, "micro avg": {"f1-score": 0.9619831226408259, "precision": 0.9619831226408259, "recall": 0.9619831226408259, "support": 151801}, "weighted avg": {"f1-score": 0.9606202750576921, "precision": 0.960238761203442, "recall": 0.9619831226408259, "support": 151801}, "\u2205": {"f1-score": 0.9764597261601524, "precision": 0.9675832803436328, "recall": 0.9855005419122895, "support": 75658}, "\u23ce": {"f1-score": 0.8866901832573347, "precision": 0.9547896979803595, "recall": 0.8276582075168648, "support": 6226}, "\u23ce\u23ce": {"f1-score": 0.7658730158730159, "precision": 0.8894009216589862, "recall": 0.6724738675958188, "support": 574}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.863996747306363, "precision": 0.9071504802561366, "recall": 0.8247622744032602, "support": 5153}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8750282997509621, "precision": 0.9367426078526417, "recall": 0.8209430756159728, "support": 4708}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 183}, "\u2423": {"f1-score": 0.9623631547673002, "precision": 0.9543565978187049, "recall": 0.9705051900302667, "support": 49229}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 10038}, "macro avg": {"f1-score": 0.6127789270289933, "precision": 0.7344470651011624, "recall": 0.5626417292199769, "support": 162873}, "micro avg": {"f1-score": 0.9281351493927048, "precision": 0.9619831226408259, "recall": 0.8965881392250403, "support": 162873}, "weighted avg": {"f1-score": 0.9189533745443913, "precision": 0.9584000766302551, "recall": 0.8965881392250403, "support": 162873}, "\u2205": {"f1-score": 0.9650785021809757, "precision": 0.9675832803436328, "recall": 0.9625866587484992, "support": 77459}, "\u23ce": {"f1-score": 0.6416386502303574, "precision": 0.9547896979803595, "recall": 0.48316924519456167, "support": 10665}, "\u23ce\u23ce": {"f1-score": 0.31293068504256183, "precision": 0.8894009216589862, "recall": 0.18986719134284308, "support": 2033}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 136}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.817150547971544, "precision": 0.9071504802561366, "recall": 0.7433968864789225, "support": 5717}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8373957317733723, "precision": 0.9367426078526417, "recall": 0.7571008814887366, "support": 5105}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 217}, "\u2423": {"f1-score": 0.9408162260621277, "precision": 0.9543565978187049, "recall": 0.9276546997262295, "support": 51503}},
  "ppcr": 0.9320206541292909
}
```
</details>
