# Train report for javascript / file:///tmp/top-repos-quality-repos-7sr8mb6_/holojs.git HEAD bdb7afb0d45a858f45a5a17b15b98291a96e7f13

### Classification report

PPCR: 0.792

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.978| 0.890| 0.986| 0.939| 23714| 26067| 0.910 |
| `␣` | 0.953| 0.993| 0.844| 0.973| 0.895| 17702| 20823| 0.850 |
| `'` | 1.000| 1.000| 0.746| 1.000| 0.855| 761| 1020| 0.746 |
| `⏎⏎⇥⁺` | 0.917| 0.970| 0.828| 0.943| 0.870| 738| 865| 0.853 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 108| 2301| 0.047 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 108| 2237| 0.048 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 96| 104| 0.923 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 67| 868| 0.077 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 145| 0.103 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 142| 0.077 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 150| 0.027 |
| `weighted avg` | 0.967| 0.975| 0.772| 0.971| 0.818| 43324| 54722| 0.792 |
| `micro avg` | 0.975| 0.975| 0.772| 0.975| 0.862| 43324| 54722| 0.792 |
| `macro avg` | 0.351| 0.358| 0.301| 0.355| 0.324| 43324| 54722| 0.792 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎⏎| ⏎| '| ⏎⏎⇥⁻| ⏎⏎⇥⁺| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁺| ⏎⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2353 |23191 |522 |0 |0 |0 |0 |1 |0 |0 |0 |0 |
|3121 |97 |17580 |0 |0 |0 |0 |25 |0 |0 |0 |0 |
|2193 |13 |91 |0 |0 |0 |0 |4 |0 |0 |0 |0 |
|2129 |9 |99 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|259 |0 |0 |0 |0 |761 |0 |0 |0 |0 |0 |0 |
|801 |13 |54 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|127 |0 |22 |0 |0 |0 |0 |716 |0 |0 |0 |0 |
|130 |1 |6 |0 |0 |0 |0 |8 |0 |0 |0 |0 |
|131 |0 |11 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8 |6 |67 |0 |0 |0 |0 |23 |0 |0 |0 |0 |
|146 |0 |0 |0 |0 |0 |0 |4 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| sample-apps/threejs/loaders/GLTFLoader.js | 261 |
| sample-apps/appcode/convex-break.js | 167 |
| sample-apps/appcode/cloth-physics.js | 121 |
| sample-apps/appcode/vr-paint.js | 105 |
| sample-apps/threejs/control/OrbitControls.js | 97 |
| windows/src/test/unittest/http-server/test-content/threejs/control/OrbitControls.js | 97 |
| sample-apps/threejs/QuickHull.js | 49 |
| sample-apps/threejs/loaders/OBJLoader.js | 34 |
| sample-apps/appcode/hololens-surface-mapping.js | 22 |
| sample-apps/appcode/ballshooter.js | 17 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 761}, "macro avg": {"f1-score": 0.35465091314556746, "precision": 0.351232511240094, "recall": 0.3582948493353438, "support": 43324}, "micro avg": {"f1-score": 0.9751638814513895, "precision": 0.9751638814513895, "recall": 0.9751638814513895, "support": 43324}, "weighted avg": {"f1-score": 0.9706476699676438, "precision": 0.9665712812270532, "recall": 0.9751638814513895, "support": 43324}, "\u2205": {"f1-score": 0.9859280673412125, "precision": 0.9940420060008572, "recall": 0.9779455174158724, "support": 23714}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.9427254772876893, "precision": 0.9167733674775929, "recall": 0.9701897018970189, "support": 738}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 67}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.9725064999723405, "precision": 0.952742250162584, "recall": 0.9931081233758897, "support": 17702}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8545760808534532, "precision": 1.0, "recall": 0.746078431372549, "support": 1020}, "macro avg": {"f1-score": 0.3235230732359111, "precision": 0.351232511240094, "recall": 0.3007047071380389, "support": 54722}, "micro avg": {"f1-score": 0.8617995634702078, "precision": 0.9751638814513895, "recall": 0.7720478052702752, "support": 54722}, "weighted avg": {"f1-score": 0.8176139319469173, "precision": 0.8691870510658957, "recall": 0.7720478052702752, "support": 54722}, "\u2205": {"f1-score": 0.9389639046905683, "precision": 0.9940420060008572, "recall": 0.8896689300648329, "support": 26067}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2237}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 104}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2301}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.8699878493317134, "precision": 0.9167733674775929, "recall": 0.8277456647398844, "support": 865}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 868}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 150}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 145}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 142}, "\u2423": {"f1-score": 0.8952259707192871, "precision": 0.952742250162584, "recall": 0.8442587523411612, "support": 20823}},
  "ppcr": 0.7917108292825554
}
```
</details>
