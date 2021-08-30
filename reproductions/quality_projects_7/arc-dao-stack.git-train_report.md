# Train report for javascript / file:///tmp/top-repos-quality-repos-9teujqd0/arc-dao-stack.git HEAD 20eb4106035eb5b1fc17964dbb3f8f2dd8bb5371

### Classification report

PPCR: 0.832

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 0.991| 0.941| 0.982| 0.956| 46764| 49273| 0.949 |
| `␣` | 0.958| 0.912| 0.726| 0.934| 0.826| 10877| 13655| 0.797 |
| `"` | 0.857| 1.000| 0.537| 0.923| 0.660| 854| 1591| 0.537 |
| `⏎` | 0.941| 0.685| 0.066| 0.793| 0.124| 397| 4110| 0.097 |
| `⏎⏎` | 0.879| 0.740| 0.272| 0.803| 0.416| 315| 856| 0.368 |
| `'` | 1.000| 0.466| 0.130| 0.636| 0.230| 266| 956| 0.278 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 93| 543| 0.171 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 91| 478| 0.190 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 135| 0.200 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 129| 0.178 |
| `weighted avg` | 0.964| 0.967| 0.805| 0.965| 0.844| 59707| 71726| 0.832 |
| `micro avg` | 0.967| 0.967| 0.805| 0.967| 0.879| 59707| 71726| 0.832 |
| `macro avg` | 0.561| 0.479| 0.267| 0.507| 0.321| 59707| 71726| 0.832 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2509 |46359 |405 |0 |0 |0 |0 |0 |0 |0 |0 |
|2778 |950 |9918 |0 |0 |0 |9 |0 |0 |0 |0 |
|3713 |92 |11 |272 |0 |0 |22 |0 |0 |0 |0 |
|737 |0 |0 |0 |854 |0 |0 |0 |0 |0 |0 |
|690 |0 |0 |0 |142 |124 |0 |0 |0 |0 |0 |
|541 |71 |9 |2 |0 |0 |233 |0 |0 |0 |0 |
|450 |83 |10 |0 |0 |0 |0 |0 |0 |0 |0 |
|387 |77 |2 |11 |0 |0 |1 |0 |0 |0 |0 |
|106 |20 |0 |3 |0 |0 |0 |0 |0 |0 |0 |
|108 |23 |3 |1 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/absolutevote.js | 248 |
| test/genesisprotocol.js | 246 |
| test/vestingscheme.js | 208 |
| test/controller.js | 200 |
| test/ucontroller.js | 200 |
| test/helpers.js | 143 |
| test/quorumvote.js | 132 |
| test/reputation.js | 115 |
| test/daotoken.js | 59 |
| test/contributionreward.js | 59 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9232432432432433, "precision": 0.857429718875502, "recall": 1.0, "support": 854}, "\u0027": {"f1-score": 0.635897435897436, "precision": 1.0, "recall": 0.46616541353383456, "support": 266}, "macro avg": {"f1-score": 0.5071486668070972, "precision": 0.560776866357531, "recall": 0.47941582908782704, "support": 59707}, "micro avg": {"f1-score": 0.9673907582025558, "precision": 0.9673907582025558, "recall": 0.9673907582025558, "support": 59707}, "weighted avg": {"f1-score": 0.9646726723075953, "precision": 0.9636550894230125, "recall": 0.9673907582025558, "support": 59707}, "\u2205": {"f1-score": 0.9817765965332119, "precision": 0.9723964341898269, "recall": 0.9913394919168591, "support": 46764}, "\u23ce": {"f1-score": 0.793002915451895, "precision": 0.9411764705882353, "recall": 0.6851385390428212, "support": 397}, "\u23ce\u23ce": {"f1-score": 0.8034482758620689, "precision": 0.879245283018868, "recall": 0.7396825396825397, "support": 315}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 93}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 91}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u2423": {"f1-score": 0.9341182010831175, "precision": 0.957520756902877, "recall": 0.9118323067022157, "support": 10877}},
  "cl_report_full": {"\"": {"f1-score": 0.6602241979126401, "precision": 0.857429718875502, "recall": 0.5367693274670019, "support": 1591}, "\u0027": {"f1-score": 0.22962962962962966, "precision": 1.0, "recall": 0.1297071129707113, "support": 956}, "macro avg": {"f1-score": 0.3211639481645219, "precision": 0.560776866357531, "recall": 0.26720402093402185, "support": 71726}, "micro avg": {"f1-score": 0.8789269057238289, "precision": 0.9673907582025558, "recall": 0.8052867858238296, "support": 71726}, "weighted avg": {"f1-score": 0.8440025789889198, "precision": 0.9470606945383383, "recall": 0.8052867858238296, "support": 71726}, "\u2205": {"f1-score": 0.9563683624210917, "precision": 0.9723964341898269, "recall": 0.9408601059403731, "support": 49273}, "\u23ce": {"f1-score": 0.1236644691975449, "precision": 0.9411764705882353, "recall": 0.06618004866180048, "support": 4110}, "\u23ce\u23ce": {"f1-score": 0.415700267618198, "precision": 0.879245283018868, "recall": 0.272196261682243, "support": 856}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 543}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 135}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 478}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 129}, "\u2423": {"f1-score": 0.8260525548661142, "precision": 0.957520756902877, "recall": 0.7263273526180886, "support": 13655}},
  "ppcr": 0.8324317541756128
}
```
</details>