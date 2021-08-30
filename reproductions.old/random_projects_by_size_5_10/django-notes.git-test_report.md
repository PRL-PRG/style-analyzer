# Test report for javascript / file:///tmp/top-repos-quality-repos-8qd299gv/django-notes.git HEAD 49305b2696dc08fa8216d014b8406547d652aeda

### Classification report

PPCR: 0.956

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.963| 0.973| 0.972| 0.968| 0.967| 6722| 6733| 0.998 |
| `␣` | 0.899| 0.969| 0.947| 0.932| 0.922| 3571| 3652| 0.978 |
| `'` | 0.831| 0.973| 0.973| 0.897| 0.897| 2100| 2100| 1.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 466| 466| 1.000 |
| `⏎` | 0.904| 0.508| 0.230| 0.650| 0.367| 333| 735| 0.453 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.659| 0.817| 0.817| 0.730| 0.730| 289| 289| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.988| 0.924| 0.910| 0.955| 0.948| 275| 279| 0.986 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 128| 129| 0.992 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 152| 0.118 |
| `macro avg` | 0.583| 0.574| 0.539| 0.570| 0.537| 13902| 14535| 0.956 |
| `weighted avg` | 0.877| 0.914| 0.874| 0.893| 0.861| 13902| 14535| 0.956 |
| `micro avg` | 0.914| 0.914| 0.874| 0.914| 0.894| 13902| 14535| 0.956 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| "| ⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|11 |6543 |167 |2 |0 |7 |3 |0 |0 |0 |
|81 |110 |3459 |0 |2 |0 |0 |0 |0 |0 |
|0 |35 |21 |2044 |0 |0 |0 |0 |0 |0 |
|402 |30 |128 |0 |169 |6 |0 |0 |0 |0 |
|0 |4 |47 |0 |2 |236 |0 |0 |0 |0 |
|4 |13 |8 |0 |0 |0 |254 |0 |0 |0 |
|134 |2 |4 |0 |12 |0 |0 |0 |0 |0 |
|0 |49 |4 |413 |0 |0 |0 |0 |0 |0 |
|1 |7 |10 |0 |2 |109 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 466}, "\u0027": {"f1-score": 0.896687870146962, "precision": 0.8312322082147214, "recall": 0.9733333333333334, "support": 2100}, "macro avg": {"f1-score": 0.5702026809757478, "precision": 0.5827362424525409, "recall": 0.5736770508772181, "support": 13902}, "micro avg": {"f1-score": 0.9138972809667674, "precision": 0.9138972809667674, "recall": 0.9138972809667674, "support": 13902}, "weighted avg": {"f1-score": 0.8927779968211361, "precision": 0.8771006570202434, "recall": 0.9138972809667674, "support": 13902}, "\u2205": {"f1-score": 0.9682574916759157, "precision": 0.9631974090976004, "recall": 0.9733710205296043, "support": 6722}, "\u23ce": {"f1-score": 0.6499999999999999, "precision": 0.9037433155080213, "recall": 0.5075075075075075, "support": 333}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 128}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.7295208655332303, "precision": 0.659217877094972, "recall": 0.8166089965397924, "support": 289}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9548872180451128, "precision": 0.9883268482490273, "recall": 0.9236363636363636, "support": 275}, "\u2423": {"f1-score": 0.9324706833805095, "precision": 0.8989085239085239, "recall": 0.9686362363483618, "support": 3571}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 466}, "\u0027": {"f1-score": 0.896687870146962, "precision": 0.8312322082147214, "recall": 0.9733333333333334, "support": 2100}, "macro avg": {"f1-score": 0.5367149274959474, "precision": 0.5827362424525409, "recall": 0.5388001771630029, "support": 14535}, "micro avg": {"f1-score": 0.8935541723810528, "precision": 0.9138972809667674, "recall": 0.8740970072239422, "support": 14535}, "weighted avg": {"f1-score": 0.8607037484359013, "precision": 0.8699083739978885, "recall": 0.8740970072239422, "support": 14535}, "\u2205": {"f1-score": 0.967470057666716, "precision": 0.9631974090976004, "recall": 0.9717807812267935, "support": 6733}, "\u23ce": {"f1-score": 0.3665943600867679, "precision": 0.9037433155080213, "recall": 0.22993197278911565, "support": 735}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 129}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 152}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.7295208655332303, "precision": 0.659217877094972, "recall": 0.8166089965397924, "support": 289}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9477611940298508, "precision": 0.9883268482490273, "recall": 0.910394265232975, "support": 279}, "\u2423": {"f1-score": 0.9223999999999999, "precision": 0.8989085239085239, "recall": 0.9471522453450164, "support": 3652}},
  "ppcr": 0.9564499484004128
}
```
</details>