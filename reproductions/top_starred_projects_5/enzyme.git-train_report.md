# Train report for javascript / file:///tmp/top-repos-quality-repos-kkd3jyau/enzyme.git HEAD 8c55734eb19a2f7555eb3d695ae11401fa1f9847

### Classification report

PPCR: 0.865

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 0.990| 0.946| 0.986| 0.964| 17411| 18210| 0.956 |
| `␣` | 0.961| 0.975| 0.791| 0.968| 0.868| 8091| 9978| 0.811 |
| `'` | 1.000| 1.000| 0.963| 1.000| 0.981| 2105| 2185| 0.963 |
| `⏎␣⁻␣⁻` | 0.971| 0.971| 0.907| 0.971| 0.938| 1291| 1382| 0.934 |
| `⏎␣⁺␣⁺` | 0.952| 0.913| 0.823| 0.932| 0.883| 1283| 1424| 0.901 |
| `⏎` | 0.964| 0.720| 0.234| 0.824| 0.377| 788| 2420| 0.326 |
| `⏎⏎` | 0.957| 0.982| 0.579| 0.969| 0.721| 386| 655| 0.589 |
| `weighted avg` | 0.976| 0.976| 0.844| 0.975| 0.891| 31355| 36254| 0.865 |
| `macro avg` | 0.970| 0.936| 0.749| 0.950| 0.819| 31355| 36254| 0.865 |
| `micro avg` | 0.976| 0.976| 0.844| 0.976| 0.905| 31355| 36254| 0.865 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|799 |17233 |156 |0 |0 |5 |17 |0 |
|1887 |113 |7892 |10 |0 |54 |19 |3 |
|1632 |75 |130 |567 |0 |0 |2 |14 |
|80 |0 |0 |0 |2105 |0 |0 |0 |
|141 |72 |37 |2 |0 |1172 |0 |0 |
|91 |34 |0 |3 |0 |0 |1254 |0 |
|269 |0 |1 |6 |0 |0 |0 |379 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/enzyme/src/ShallowWrapper.js | 163 |
| packages/enzyme-adapter-react-16/src/ReactSixteenAdapter.js | 130 |
| packages/enzyme/src/selectors.js | 40 |
| env.js | 39 |
| packages/enzyme/src/ReactWrapper.js | 38 |
| packages/enzyme-adapter-utils/src/Utils.js | 38 |
| packages/enzyme/src/Debug.js | 37 |
| packages/enzyme/src/Utils.js | 31 |
| packages/enzyme-adapter-react-15/src/ReactFifteenAdapter.js | 23 |
| packages/enzyme-adapter-react-14/src/ReactFourteenAdapter.js | 19 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2105}, "macro avg": {"f1-score": 0.9501711832449801, "precision": 0.9696866821459346, "recall": 0.9359162641771379, "support": 31355}, "micro avg": {"f1-score": 0.9759846914367725, "precision": 0.9759846914367725, "recall": 0.9759846914367725, "support": 31355}, "weighted avg": {"f1-score": 0.9754619892765009, "precision": 0.9759112986382695, "recall": 0.9759846914367725, "support": 31355}, "\u2205": {"f1-score": 0.9864903543419772, "precision": 0.983225880070748, "recall": 0.9897765780253862, "support": 17411}, "\u23ce": {"f1-score": 0.8241279069767442, "precision": 0.9642857142857143, "recall": 0.7195431472081218, "support": 788}, "\u23ce\u23ce": {"f1-score": 0.969309462915601, "precision": 0.9570707070707071, "recall": 0.9818652849740933, "support": 386}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9323786793953858, "precision": 0.9520714865962632, "recall": 0.9134840218238504, "support": 1283}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9709639953542393, "precision": 0.9705882352941176, "recall": 0.9713400464756003, "support": 1291}, "\u2423": {"f1-score": 0.9679278837309131, "precision": 0.9605647517039922, "recall": 0.9754047707329131, "support": 8091}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9813519813519813, "precision": 1.0, "recall": 0.9633867276887872, "support": 2185}, "macro avg": {"f1-score": 0.8189030228735759, "precision": 0.9696866821459346, "recall": 0.7491446781042475, "support": 36254}, "micro avg": {"f1-score": 0.9052640920587495, "precision": 0.9759846914367725, "recall": 0.8440999613835715, "support": 36254}, "weighted avg": {"f1-score": 0.8909647942212392, "precision": 0.9745576722122536, "recall": 0.8440999613835715, "support": 36254}, "\u2205": {"f1-score": 0.9644346195819459, "precision": 0.983225880070748, "recall": 0.9463481603514552, "support": 18210}, "\u23ce": {"f1-score": 0.37699468085106386, "precision": 0.9642857142857143, "recall": 0.23429752066115703, "support": 2420}, "\u23ce\u23ce": {"f1-score": 0.7212178877259752, "precision": 0.9570707070707071, "recall": 0.5786259541984733, "support": 655}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8828625235404896, "precision": 0.9520714865962632, "recall": 0.8230337078651685, "support": 1424}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9379207180254302, "precision": 0.9705882352941176, "recall": 0.9073806078147613, "support": 1382}, "\u2423": {"f1-score": 0.8675387490381444, "precision": 0.9605647517039922, "recall": 0.7909400681499299, "support": 9978}},
  "ppcr": 0.864870083301153
}
```
</details>
