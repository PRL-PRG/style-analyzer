# Train report for javascript / file:///tmp/top-repos-quality-repos-sv4mmyyk/create-react-app.git HEAD 749a76d291d6b07681d0941eeb23112c6970d3c4

### Classification report

PPCR: 0.860

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.926| 0.985| 0.949| 0.955| 0.937| 9262| 9617| 0.963 |
| `␣` | 0.909| 0.909| 0.674| 0.909| 0.774| 3004| 4054| 0.741 |
| `'` | 1.000| 1.000| 0.987| 1.000| 0.993| 2818| 2856| 0.987 |
| `⏎` | 0.957| 0.825| 0.395| 0.886| 0.559| 926| 1934| 0.479 |
| `⏎␣⁺␣⁺` | 0.934| 0.709| 0.620| 0.806| 0.745| 779| 891| 0.874 |
| `⏎␣⁻␣⁻` | 1.000| 0.696| 0.612| 0.821| 0.760| 751| 854| 0.879 |
| `⏎⏎` | 0.803| 0.599| 0.290| 0.686| 0.426| 197| 407| 0.484 |
| `weighted avg` | 0.939| 0.938| 0.807| 0.935| 0.852| 17737| 20613| 0.860 |
| `macro avg` | 0.933| 0.818| 0.647| 0.866| 0.742| 17737| 20613| 0.860 |
| `micro avg` | 0.938| 0.938| 0.807| 0.938| 0.867| 17737| 20613| 0.860 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|355 |9124 |127 |0 |0 |11 |0 |0 |
|1050 |232 |2732 |0 |12 |28 |0 |0 |
|38 |0 |0 |2818 |0 |0 |0 |0 |
|1008 |69 |64 |0 |764 |0 |0 |29 |
|112 |187 |40 |0 |0 |552 |0 |0 |
|103 |213 |4 |0 |11 |0 |523 |0 |
|210 |30 |38 |0 |11 |0 |0 |118 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/create-react-app/createReactApp.js | 445 |
| packages/react-scripts/config/webpack.config.js | 94 |
| packages/react-scripts/fixtures/kitchensink/template/integration/webpack.test.js | 53 |
| packages/babel-plugin-named-asset-import/index.js | 50 |
| packages/babel-preset-react-app/create.js | 48 |
| packages/cra-template/template/src/serviceWorker.js | 47 |
| packages/react-scripts/fixtures/kitchensink/template/integration/syntax.test.js | 44 |
| packages/eslint-config-react-app/index.js | 39 |
| docusaurus/website/src/pages/index.js | 35 |
| packages/react-scripts/fixtures/kitchensink/template/integration/env.test.js | 34 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2818}, "macro avg": {"f1-score": 0.866154104107065, "precision": 0.9327286561899962, "recall": 0.8176569718015604, "support": 17737}, "micro avg": {"f1-score": 0.9376444720076675, "precision": 0.9376444720076675, "recall": 0.9376444720076675, "support": 17737}, "weighted avg": {"f1-score": 0.9353749455004475, "precision": 0.9385664585881986, "recall": 0.9376444720076675, "support": 17737}, "\u2205": {"f1-score": 0.9545430768426009, "precision": 0.9258244545915779, "recall": 0.9851004102785575, "support": 9262}, "\u23ce": {"f1-score": 0.8863109048723898, "precision": 0.9573934837092731, "recall": 0.8250539956803455, "support": 926}, "\u23ce\u23ce": {"f1-score": 0.686046511627907, "precision": 0.8027210884353742, "recall": 0.5989847715736041, "support": 197}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8058394160583942, "precision": 0.934010152284264, "recall": 0.7086007702182285, "support": 779}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8210361067503925, "precision": 1.0, "recall": 0.6964047936085219, "support": 751}, "\u2423": {"f1-score": 0.9093027125977701, "precision": 0.9091514143094842, "recall": 0.9094540612516645, "support": 3004}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9933027846316532, "precision": 1.0, "recall": 0.9866946778711485, "support": 2856}, "macro avg": {"f1-score": 0.7420488923177222, "precision": 0.9327286561899962, "recall": 0.6466052700963499, "support": 20613}, "micro avg": {"f1-score": 0.8673272490221642, "precision": 0.9376444720076675, "recall": 0.8068209382428565, "support": 20613}, "weighted avg": {"f1-score": 0.8516401844972856, "precision": 0.9367808247023756, "recall": 0.8068209382428565, "support": 20613}, "\u2205": {"f1-score": 0.9371405094494659, "precision": 0.9258244545915779, "recall": 0.9487366122491422, "support": 9617}, "\u23ce": {"f1-score": 0.5592972181551977, "precision": 0.9573934837092731, "recall": 0.39503619441571874, "support": 1934}, "\u23ce\u23ce": {"f1-score": 0.42599277978339345, "precision": 0.8027210884353742, "recall": 0.28992628992628994, "support": 407}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7449392712550607, "precision": 0.934010152284264, "recall": 0.6195286195286195, "support": 891}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7596223674655047, "precision": 1.0, "recall": 0.6124121779859485, "support": 854}, "\u2423": {"f1-score": 0.7740473154837797, "precision": 0.9091514143094842, "recall": 0.6739023186975827, "support": 4054}},
  "ppcr": 0.860476398389366
}
```
</details>
