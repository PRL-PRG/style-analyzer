# Train report for javascript / file:///tmp/top-repos-quality-repos-c25srffy/react.git HEAD 5634ed16aaba4a507844f0502953abcf40e3a165

### Classification report

PPCR: 0.956

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.985| 0.993| 0.983| 0.989| 0.984| 352355| 356202| 0.989 |
| `␣` | 0.965| 0.983| 0.934| 0.974| 0.949| 113662| 119733| 0.949 |
| `'` | 1.000| 1.000| 0.981| 1.000| 0.990| 50345| 51342| 0.981 |
| `⏎` | 0.866| 0.951| 0.780| 0.907| 0.821| 36294| 44242| 0.820 |
| `⏎␣⁻␣⁻` | 0.968| 0.921| 0.885| 0.944| 0.925| 22651| 23559| 0.961 |
| `⏎␣⁺␣⁺` | 0.961| 0.806| 0.685| 0.877| 0.800| 20840| 24537| 0.849 |
| `⏎⏎` | 0.961| 0.484| 0.303| 0.644| 0.461| 7809| 12470| 0.626 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 3860| 3860| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.933| 0.403| 0.324| 0.563| 0.481| 553| 688| 0.804 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 116| 148| 0.784 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 59| 0.373 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.973| 0.973| 0.930| 0.972| 0.946| 608507| 636840| 0.956 |
| `macro avg` | 0.720| 0.629| 0.573| 0.658| 0.618| 608507| 636840| 0.956 |
| `micro avg` | 0.973| 0.973| 0.930| 0.973| 0.951| 608507| 636840| 0.956 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3847 |350025 |1498 |7 |0 |403 |422 |0 |0 |0 |0 |0 |
|6071 |957 |111780 |901 |0 |11 |13 |0 |0 |0 |0 |0 |
|7948 |238 |1117 |34522 |0 |252 |14 |150 |0 |1 |0 |0 |
|997 |0 |0 |0 |50345 |0 |0 |0 |0 |0 |0 |0 |
|3697 |3105 |926 |6 |0 |16800 |3 |0 |0 |0 |0 |0 |
|908 |1010 |368 |396 |0 |0 |20861 |1 |0 |15 |0 |0 |
|4661 |5 |43 |3983 |0 |0 |0 |3778 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |3860 |0 |0 |0 |
|135 |69 |6 |21 |0 |0 |234 |0 |0 |223 |0 |0 |
|37 |0 |0 |19 |0 |0 |0 |3 |0 |0 |0 |0 |
|32 |14 |85 |0 |0 |17 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/eslint-plugin-react-hooks/__tests__/ESLintRuleExhaustiveDeps-test.js | 990 |
| packages/react/src/__tests__/ReactProfiler-test.internal.js | 644 |
| packages/react-reconciler/src/__tests__/ReactHooksWithNoopRenderer-test.internal.js | 407 |
| packages/eslint-plugin-react-hooks/src/ExhaustiveDeps.js | 383 |
| packages/react-reconciler/src/__tests__/ReactSuspenseWithNoopRenderer-test.internal.js | 367 |
| fixtures/attribute-behavior/src/attributes.js | 342 |
| packages/react-reconciler/src/__tests__/ReactIncremental-test.internal.js | 337 |
| packages/react-reconciler/src/__tests__/ReactSuspenseList-test.internal.js | 321 |
| packages/react-reconciler/src/__tests__/ReactNewContext-test.internal.js | 310 |
| packages/react-dom/src/__tests__/ReactDOMServerPartialHydration-test.internal.js | 272 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3860}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 50345}, "macro avg": {"f1-score": 0.6581153543368549, "precision": 0.7199233636822958, "recall": 0.6285148775699589, "support": 608507}, "micro avg": {"f1-score": 0.9731917627899104, "precision": 0.9731917627899104, "recall": 0.9731917627899104, "support": 608507}, "weighted avg": {"f1-score": 0.9717857566497771, "precision": 0.9733912096578411, "recall": 0.9731917627899104, "support": 608507}, "\u2205": {"f1-score": 0.989081322109475, "precision": 0.9848124628963235, "recall": 0.9933873508251622, "support": 352355}, "\u23ce": {"f1-score": 0.9066960826800089, "precision": 0.866189938527161, "recall": 0.9511765030032512, "support": 36294}, "\u23ce\u23ce": {"f1-score": 0.6435567668852739, "precision": 0.9608341810783316, "recall": 0.48380074273274426, "support": 7809}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8767580826135741, "precision": 0.9609334782360007, "recall": 0.8061420345489443, "support": 20840}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9439793655821531, "precision": 0.9681626212465773, "recall": 0.9209747913999382, "support": 22651}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.5631313131313133, "precision": 0.9330543933054394, "recall": 0.40325497287522605, "support": 553}, "\u2423": {"f1-score": 0.9741813190404601, "precision": 0.9650932888977146, "recall": 0.9834421354542415, "support": 113662}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3860}, "\u0027": {"f1-score": 0.9901954035422424, "precision": 1.0, "recall": 0.9805812005765261, "support": 51342}, "macro avg": {"f1-score": 0.6175334323392158, "precision": 0.7199233636822958, "recall": 0.5728642255089464, "support": 636840}, "micro avg": {"f1-score": 0.951050590718892, "precision": 0.9731917627899104, "recall": 0.9298944789900132, "support": 636840}, "weighted avg": {"f1-score": 0.9461599589708262, "precision": 0.9717991497684988, "recall": 0.9298944789900132, "support": 636840}, "\u2205": {"f1-score": 0.9837344106797821, "precision": 0.9848124628963235, "recall": 0.9826587161217512, "support": 356202}, "\u23ce": {"f1-score": 0.8210043164441062, "precision": 0.866189938527161, "recall": 0.7802992631436192, "support": 44242}, "\u23ce\u23ce": {"f1-score": 0.46067552737471035, "precision": 0.9608341810783316, "recall": 0.30296712109061746, "support": 12470}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7996192289386007, "precision": 0.9609334782360007, "recall": 0.6846802787626849, "support": 24537}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 148}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9249767215004656, "precision": 0.9681626212465773, "recall": 0.885479010144743, "support": 23559}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.4811218985976268, "precision": 0.9330543933054394, "recall": 0.3241279069767442, "support": 688}, "\u2423": {"f1-score": 0.9490736809930548, "precision": 0.9650932888977146, "recall": 0.9335772092906718, "support": 119733}},
  "ppcr": 0.9555100182149362
}
```
</details>
