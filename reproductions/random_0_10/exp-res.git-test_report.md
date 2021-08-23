# Test report for javascript / file:///tmp/top-repos-quality-repos-qrf_bazw/exp-res.git HEAD d0161687919b359f77698a7bac62ec9f5baf2d3e

### Classification report

PPCR: 0.961

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.941| 0.987| 0.965| 0.964| 0.953| 76272| 78036| 0.977 |
| `∅` | 0.989| 0.970| 0.966| 0.979| 0.977| 71914| 72181| 0.996 |
| `⏎` | 0.753| 0.785| 0.605| 0.769| 0.671| 7582| 9844| 0.770 |
| `"` | 0.927| 0.848| 0.833| 0.886| 0.878| 6372| 6488| 0.982 |
| `'` | 0.847| 0.896| 0.848| 0.871| 0.847| 5293| 5594| 0.946 |
| `⏎⏎` | 0.858| 0.580| 0.430| 0.692| 0.573| 4426| 5967| 0.742 |
| `⏎⏎⇥⁺` | 0.986| 0.944| 0.893| 0.965| 0.938| 2839| 2999| 0.947 |
| `⏎⏎⇥⁻` | 0.930| 0.868| 0.740| 0.898| 0.824| 2629| 3083| 0.853 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 417| 586| 0.712 |
| `⏎␣⁻␣⁻` | 0.760| 0.624| 0.530| 0.685| 0.624| 356| 419| 0.850 |
| `⏎␣⁺␣⁺` | 0.730| 0.899| 0.715| 0.806| 0.722| 346| 435| 0.795 |
| `⏎⇥⁻` | 0.180| 0.070| 0.061| 0.100| 0.092| 316| 358| 0.883 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 69| 83| 0.831 |
| `⏎⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 31| 59| 0.525 |
| `⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 22| 0.909 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 7| 0.429 |
| `macro avg` | 0.556| 0.529| 0.474| 0.538| 0.506| 178885| 186161| 0.961 |
| `weighted avg` | 0.943| 0.946| 0.909| 0.943| 0.920| 178885| 186161| 0.961 |
| `micro avg` | 0.946| 0.946| 0.909| 0.946| 0.927| 178885| 186161| 0.961 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| '| "| ⏎⏎⇥⁻| ⏎⏎⇥⁺| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎⇥⁻⇥⁻| ⏎⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⇥| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|267 |69726 |2136 |6 |18 |11 |0 |9 |2 |0 |6 |0 |0 |0 |0 |0 |0 |
|1764 |529 |75310 |339 |19 |0 |0 |15 |4 |42 |14 |0 |0 |0 |0 |0 |0 |
|2262 |35 |1233 |5954 |321 |2 |0 |8 |5 |11 |12 |0 |1 |0 |0 |0 |0 |
|1541 |42 |360 |1448 |2566 |0 |0 |0 |7 |3 |0 |0 |0 |0 |0 |0 |0 |
|301 |91 |24 |14 |0 |4741 |423 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|116 |40 |66 |21 |0 |841 |5404 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|454 |4 |185 |44 |27 |1 |0 |2282 |0 |0 |33 |0 |53 |0 |0 |0 |0 |
|160 |0 |100 |5 |0 |0 |0 |0 |2679 |55 |0 |0 |0 |0 |0 |0 |0 |
|89 |0 |11 |13 |0 |1 |0 |0 |10 |311 |0 |0 |0 |0 |0 |0 |0 |
|63 |0 |5 |23 |0 |1 |1 |61 |0 |0 |222 |0 |43 |0 |0 |0 |0 |
|169 |15 |374 |15 |0 |0 |0 |0 |9 |4 |0 |0 |0 |0 |0 |0 |0 |
|42 |23 |207 |7 |0 |0 |0 |55 |0 |0 |2 |0 |22 |0 |0 |0 |0 |
|28 |0 |2 |1 |0 |0 |0 |24 |0 |0 |1 |0 |3 |0 |0 |0 |0 |
|14 |0 |8 |20 |41 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |0 |1 |0 |0 |2 |0 |0 |0 |0 |0 |0 |
|2 |1 |19 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.8859016393442624, "precision": 0.9272477693891558, "recall": 0.8480853735091023, "support": 6372}, "\u0027": {"f1-score": 0.8706271233128271, "precision": 0.8469096105752054, "recall": 0.8957113168335538, "support": 5293}, "macro avg": {"f1-score": 0.538334761724501, "precision": 0.5563059263587506, "recall": 0.5293442555676063, "support": 178885}, "micro avg": {"f1-score": 0.9459541045923359, "precision": 0.9459541045923359, "recall": 0.9459541045923359, "support": 178885}, "weighted avg": {"f1-score": 0.9430973753410234, "precision": 0.9425130488028491, "recall": 0.9459541045923359, "support": 178885}, "\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u2205": {"f1-score": 0.9791602303047325, "precision": 0.9889371117351715, "recall": 0.9695747698640043, "support": 71914}, "\u23ce": {"f1-score": 0.7686547895688097, "precision": 0.7527180783817952, "recall": 0.7852809285149037, "support": 7582}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 417}, "\u23ce\u21e5\u207b": {"f1-score": 0.10045662100456619, "precision": 0.18032786885245902, "recall": 0.06962025316455696, "support": 316}, "\u23ce\u23ce": {"f1-score": 0.6918306821245619, "precision": 0.857620320855615, "recall": 0.5797559873474921, "support": 4426}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.9645364536453644, "precision": 0.9863770250368189, "recall": 0.9436421275096865, "support": 2839}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.8977183320220299, "precision": 0.9295315682281059, "recall": 0.8680106504374286, "support": 2629}, "\u23ce\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 69}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8056994818652851, "precision": 0.7300469483568075, "recall": 0.8988439306358381, "support": 346}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6851851851851851, "precision": 0.7602739726027398, "recall": 0.6235955056179775, "support": 356}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.9635856492143918, "precision": 0.940904547726137, "recall": 0.9873872456471575, "support": 76272}},
  "cl_report_full": {"\"": {"f1-score": 0.8775576485872036, "precision": 0.9272477693891558, "recall": 0.8329223181257707, "support": 6488}, "\u0027": {"f1-score": 0.8472122944960686, "precision": 0.8469096105752054, "recall": 0.8475151948516267, "support": 5594}, "macro avg": {"f1-score": 0.5061685445280848, "precision": 0.5563059263587506, "recall": 0.4741296465254864, "support": 186161}, "micro avg": {"f1-score": 0.9270995984067761, "precision": 0.9459541045923359, "recall": 0.908982010195476, "support": 186161}, "weighted avg": {"f1-score": 0.9202491730984375, "precision": 0.9379636077000084, "recall": 0.908982010195476, "support": 186161}, "\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u2205": {"f1-score": 0.9773279976451955, "precision": 0.9889371117351715, "recall": 0.9659882794641249, "support": 72181}, "\u23ce": {"f1-score": 0.6707220907964403, "precision": 0.7527180783817952, "recall": 0.6048354327509142, "support": 9844}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 586}, "\u23ce\u21e5\u207b": {"f1-score": 0.09166666666666667, "precision": 0.18032786885245902, "recall": 0.061452513966480445, "support": 358}, "\u23ce\u23ce": {"f1-score": 0.5728317892621945, "precision": 0.857620320855615, "recall": 0.4300318417965477, "support": 5967}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.9375328083989501, "precision": 0.9863770250368189, "recall": 0.893297765921974, "support": 2999}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.8241242325749367, "precision": 0.9295315682281059, "recall": 0.7401881284463185, "support": 3083}, "\u23ce\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 83}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7224157955865272, "precision": 0.7300469483568075, "recall": 0.7149425287356321, "support": 435}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6244725738396625, "precision": 0.7602739726027398, "recall": 0.5298329355608592, "support": 419}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.9528328145955111, "precision": 0.940904547726137, "recall": 0.965067404787534, "support": 78036}},
  "ppcr": 0.9609155515924388
}
```
</details>