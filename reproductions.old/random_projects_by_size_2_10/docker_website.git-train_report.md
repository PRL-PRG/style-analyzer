# Train report for javascript / file:///tmp/top-repos-quality-repos-8losb434/docker_website.git HEAD ecbabb3a1a20a0e6c4346dc16fda1762ce42afc0

### Classification report

PPCR: 0.668

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.982| 0.998| 0.880| 0.990| 0.928| 4098| 4650| 0.881 |
| `⏎` | 0.970| 0.987| 0.729| 0.979| 0.832| 463| 627| 0.738 |
| `␣` | 0.958| 0.741| 0.098| 0.836| 0.178| 216| 1634| 0.132 |
| `⏎␣⁺␣⁺` | 1.000| 0.982| 0.851| 0.991| 0.919| 168| 194| 0.866 |
| `'` | 1.000| 1.000| 0.394| 1.000| 0.566| 140| 355| 0.394 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 187| 0.139 |
| `macro avg` | 0.818| 0.785| 0.492| 0.799| 0.570| 5111| 7647| 0.668 |
| `micro avg` | 0.981| 0.981| 0.656| 0.981| 0.786| 5111| 7647| 0.668 |
| `weighted avg` | 0.976| 0.981| 0.656| 0.978| 0.720| 5111| 7647| 0.668 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|552 |4091 |7 |0 |0 |0 |0 |
|1418 |47 |160 |9 |0 |0 |0 |
|164 |6 |0 |457 |0 |0 |0 |
|215 |0 |0 |0 |140 |0 |0 |
|26 |0 |0 |3 |0 |165 |0 |
|161 |24 |0 |2 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| django/app_website/static/app_website/js/kernel.js | 43 |
| django/app_website/static/app_website/js/bin/shell.js | 15 |
| django/app_website/static/app_website/js/bin/pong.js | 10 |
| django/app_website/static/app_website/js/bin/help.js | 8 |
| django/app_website/static/app_website/js/boot.js | 8 |
| django/app_website/static/app_website/js/bin/hexdump.js | 8 |
| django/app_website/static/app_website/js/bin/whoami.js | 3 |
| django/app_website/static/app_website/js/error.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 140}, "macro avg": {"f1-score": 0.7991541238822267, "precision": 0.8183142920893586, "recall": 0.784702747380572, "support": 5111}, "micro avg": {"f1-score": 0.9808256701232636, "precision": 0.9808256701232636, "recall": 0.9808256701232636, "support": 5111}, "weighted avg": {"f1-score": 0.9775773405060487, "precision": 0.9756362915079071, "recall": 0.9808256701232636, "support": 5111}, "\u2205": {"f1-score": 0.9898378901524315, "precision": 0.9815259117082533, "recall": 0.9982918496827721, "support": 4098}, "\u23ce": {"f1-score": 0.9785867237687366, "precision": 0.970276008492569, "recall": 0.9870410367170627, "support": 463}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9909909909909909, "precision": 1.0, "recall": 0.9821428571428571, "support": 168}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u2423": {"f1-score": 0.835509138381201, "precision": 0.9580838323353293, "recall": 0.7407407407407407, "support": 216}},
  "cl_report_full": {"\u0027": {"f1-score": 0.5656565656565656, "precision": 1.0, "recall": 0.39436619718309857, "support": 355}, "macro avg": {"f1-score": 0.5704755127691693, "precision": 0.8183142920893586, "recall": 0.4919089079313193, "support": 7647}, "micro avg": {"f1-score": 0.78585985264148, "precision": 0.9808256701232636, "recall": 0.6555511965476658, "support": 7647}, "weighted avg": {"f1-score": 0.7200223821519982, "precision": 0.9529184685241463, "recall": 0.6555511965476658, "support": 7647}, "\u2205": {"f1-score": 0.9278748015422997, "precision": 0.9815259117082533, "recall": 0.8797849462365591, "support": 4650}, "\u23ce": {"f1-score": 0.8324225865209471, "precision": 0.970276008492569, "recall": 0.7288676236044657, "support": 627}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9192200557103063, "precision": 1.0, "recall": 0.8505154639175257, "support": 194}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 187}, "\u2423": {"f1-score": 0.1776790671848973, "precision": 0.9580838323353293, "recall": 0.09791921664626684, "support": 1634}},
  "ppcr": 0.6683666797436904
}
```
</details>
