# Train report for javascript / file:///tmp/top-repos-quality-repos-213yo6dn/structor.git HEAD f0bf3aab11f03899b477b66c033158a73470ea3d

### Classification report

PPCR: 0.780

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.976| 0.988| 0.884| 0.982| 0.928| 52909| 59154| 0.894 |
| `␣` | 0.960| 0.989| 0.777| 0.974| 0.859| 22694| 28887| 0.786 |
| `'` | 0.960| 1.000| 0.978| 0.979| 0.969| 6705| 6855| 0.978 |
| `⏎` | 0.954| 0.854| 0.296| 0.901| 0.452| 2353| 6783| 0.347 |
| `"` | 0.999| 0.844| 0.563| 0.915| 0.720| 1798| 2696| 0.667 |
| `⏎⏎` | 0.972| 0.654| 0.331| 0.782| 0.494| 944| 1862| 0.507 |
| `⏎␣⁺␣⁺` | 0.881| 0.379| 0.075| 0.530| 0.139| 372| 1874| 0.199 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 237| 1772| 0.134 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 145| 1217| 0.119 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 112| 1167| 0.096 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 54| 496| 0.109 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 487| 0.049 |
| `weighted avg` | 0.964| 0.970| 0.757| 0.966| 0.817| 88347| 113250| 0.780 |
| `micro avg` | 0.970| 0.970| 0.757| 0.970| 0.851| 88347| 113250| 0.780 |
| `macro avg` | 0.558| 0.476| 0.325| 0.505| 0.380| 88347| 113250| 0.780 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6245 |52300 |605 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6193 |222 |22445 |27 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4430 |118 |208 |2009 |0 |0 |18 |0 |0 |0 |0 |0 |0 |
|150 |0 |0 |0 |6703 |2 |0 |0 |0 |0 |0 |0 |0 |
|898 |0 |0 |0 |280 |1518 |0 |0 |0 |0 |0 |0 |0 |
|918 |266 |3 |58 |0 |0 |617 |0 |0 |0 |0 |0 |0 |
|1502 |211 |20 |0 |0 |0 |0 |141 |0 |0 |0 |0 |0 |
|1535 |217 |18 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1055 |69 |26 |0 |0 |0 |0 |17 |0 |0 |0 |0 |0 |
|1072 |126 |13 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|442 |22 |30 |0 |0 |0 |0 |2 |0 |0 |0 |0 |0 |
|463 |18 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src-server/structor/installManager.js | 180 |
| src-server/structor/extractManager.js | 84 |
| src-client/modules/workspace/containers/Desk/index.js | 65 |
| src-client/api/utils/cookies.js | 62 |
| src-client/api/model/graphApi.js | 56 |
| src-client/modules/workspace/containers/LibraryPanel/index.js | 55 |
| src-client/modules/generator/containers/Generator/actions.js | 49 |
| src-client/modules/workspace/containers/PageTreeViewPanel/index.js | 48 |
| src-client/api/utils/printProps.js | 43 |
| postexec.js | 42 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9150090415913201, "precision": 0.9986842105263158, "recall": 0.8442714126807565, "support": 1798}, "\u0027": {"f1-score": 0.9793980128579778, "precision": 0.9599026206501504, "recall": 0.9997017151379568, "support": 6705}, "macro avg": {"f1-score": 0.5053233404314383, "precision": 0.5584997837797586, "recall": 0.47566069529593197, "support": 88347}, "micro avg": {"f1-score": 0.9704121249165224, "precision": 0.9704121249165224, "recall": 0.9704121249165224, "support": 88347}, "weighted avg": {"f1-score": 0.9661537002282624, "precision": 0.9640297020068456, "recall": 0.9704121249165224, "support": 88347}, "\u2205": {"f1-score": 0.9823625537669753, "precision": 0.9763109260953163, "recall": 0.9884896709444518, "support": 52909}, "\u23ce": {"f1-score": 0.901098901098901, "precision": 0.9539411206077872, "recall": 0.8538036549086273, "support": 2353}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u23ce": {"f1-score": 0.7815072830905636, "precision": 0.9716535433070866, "recall": 0.6536016949152542, "support": 944}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5300751879699248, "precision": 0.88125, "recall": 0.3790322580645161, "support": 372}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 237}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 145}, "\u2423": {"f1-score": 0.9744291048015976, "precision": 0.9602549841704457, "recall": 0.9890279368996211, "support": 22694}},
  "cl_report_full": {"\"": {"f1-score": 0.7201138519924098, "precision": 0.9986842105263158, "recall": 0.5630563798219584, "support": 2696}, "\u0027": {"f1-score": 0.9687816158404394, "precision": 0.9599026206501504, "recall": 0.9778264040846097, "support": 6855}, "macro avg": {"f1-score": 0.3800539364562161, "precision": 0.5584997837797586, "recall": 0.3253995599453134, "support": 113250}, "micro avg": {"f1-score": 0.8505384504729733, "precision": 0.9704121249165224, "recall": 0.7570242825607064, "support": 113250}, "weighted avg": {"f1-score": 0.8170636838367987, "precision": 0.9244629701088756, "recall": 0.7570242825607064, "support": 113250}, "\u2205": {"f1-score": 0.9279383976650728, "precision": 0.9763109260953163, "recall": 0.884132941136694, "support": 59154}, "\u23ce": {"f1-score": 0.4520193497581281, "precision": 0.9539411206077872, "recall": 0.2961816305469556, "support": 6783}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 496}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 487}, "\u23ce\u23ce": {"f1-score": 0.4941930316379656, "precision": 0.9716535433070866, "recall": 0.3313641245972073, "support": 1862}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.13864306784660765, "precision": 0.88125, "recall": 0.07524012806830309, "support": 1874}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1167}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1772}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1217}, "\u2423": {"f1-score": 0.8589579227339699, "precision": 0.9602549841704457, "recall": 0.7769931110880327, "support": 28887}},
  "ppcr": 0.7801059602649006
}
```
</details>
