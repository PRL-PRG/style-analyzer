# Train report for javascript / file:///tmp/top-repos-quality-repos-kzk3e2wu/redux-query.git HEAD 0687bf301dad8d78576ba1c7a0dcc39762651b1c

### Classification report

PPCR: 0.431

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.989| 0.991| 0.579| 0.990| 0.731| 6311| 10798| 0.584 |
| `␣` | 1.000| 0.936| 0.175| 0.967| 0.298| 1214| 6478| 0.187 |
| `⏎␣⁻␣⁻` | 0.945| 0.993| 0.826| 0.968| 0.881| 809| 972| 0.832 |
| `⏎␣⁺␣⁺` | 0.919| 0.952| 0.693| 0.935| 0.790| 715| 983| 0.727 |
| `'` | 1.000| 1.000| 0.441| 1.000| 0.612| 618| 1401| 0.441 |
| `⏎⏎` | 0.936| 1.000| 0.292| 0.967| 0.445| 206| 706| 0.292 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 1463| 0.012 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 158| 0.000 |
| `micro avg` | 0.981| 0.981| 0.422| 0.981| 0.591| 9890| 22959| 0.431 |
| `macro avg` | 0.724| 0.734| 0.376| 0.728| 0.470| 9890| 22959| 0.431 |
| `weighted avg` | 0.980| 0.981| 0.422| 0.980| 0.550| 9890| 22959| 0.431 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4487 |6255 |0 |0 |0 |33 |23 |0 |0 |
|5264 |25 |1136 |0 |0 |27 |24 |2 |0 |
|1446 |10 |0 |0 |0 |0 |0 |7 |0 |
|783 |0 |0 |0 |618 |0 |0 |0 |0 |
|268 |34 |0 |0 |0 |681 |0 |0 |0 |
|163 |1 |0 |0 |0 |0 |803 |5 |0 |
|500 |0 |0 |0 |0 |0 |0 |206 |0 |
|158 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/redux-query/src/middleware/query.js | 22 |
| packages/redux-query-react/test/components/connect-request.test.js | 19 |
| packages/redux-query-react/test/hooks/use-mutation.test.js | 15 |
| packages/redux-query/src/types.js | 13 |
| packages/redux-query-react/test/hooks/use-requests.test.js | 12 |
| packages/redux-query-react/test/hooks/use-request.test.js | 12 |
| packages/redux-query/src/actions/index.js | 11 |
| website/pages/en/index.js | 10 |
| packages/redux-query-interface-superagent/src/index.js | 7 |
| packages/redux-query-react/flow-test/hooks/use-mutation.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 618}, "macro avg": {"f1-score": 0.7284332194179376, "precision": 0.7236288331402074, "recall": 0.7339883976585907, "support": 9890}, "micro avg": {"f1-score": 0.9806875631951466, "precision": 0.9806875631951466, "recall": 0.9806875631951466, "support": 9890}, "weighted avg": {"f1-score": 0.979878622623386, "precision": 0.9795164986636801, "recall": 0.9806875631951466, "support": 9890}, "\u2205": {"f1-score": 0.99002849002849, "precision": 0.9889328063241106, "recall": 0.9911266043416257, "support": 6311}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u23ce": {"f1-score": 0.9671361502347418, "precision": 0.9363636363636364, "recall": 1.0, "support": 206}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9354395604395603, "precision": 0.9190283400809717, "recall": 0.9524475524475524, "support": 715}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9680530440024111, "precision": 0.9447058823529412, "recall": 0.992583436341162, "support": 809}, "\u2423": {"f1-score": 0.9668085106382979, "precision": 1.0, "recall": 0.9357495881383855, "support": 1214}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 158}, "\u0027": {"f1-score": 0.612184249628529, "precision": 1.0, "recall": 0.4411134903640257, "support": 1401}, "macro avg": {"f1-score": 0.46969684715101356, "precision": 0.7236288331402074, "recall": 0.3758054748343979, "support": 22959}, "micro avg": {"f1-score": 0.5905202593686262, "precision": 0.9806875631951466, "recall": 0.4224487129230367, "support": 22959}, "weighted avg": {"f1-score": 0.5499867917135709, "precision": 0.9164261573198803, "recall": 0.4224487129230367, "support": 22959}, "\u2205": {"f1-score": 0.7305962740174036, "precision": 0.9889328063241106, "recall": 0.5792739396184479, "support": 10798}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1463}, "\u23ce\u23ce": {"f1-score": 0.44492440604751615, "precision": 0.9363636363636364, "recall": 0.29178470254957506, "support": 706}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7900232018561485, "precision": 0.9190283400809717, "recall": 0.6927772126144456, "support": 983}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8814489571899011, "precision": 0.9447058823529412, "recall": 0.8261316872427984, "support": 972}, "\u2423": {"f1-score": 0.2983976884686104, "precision": 1.0, "recall": 0.1753627662858907, "support": 6478}},
  "ppcr": 0.43076789058756915
}
```
</details>
