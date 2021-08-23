# Train report for javascript / file:///tmp/top-repos-quality-repos-7z_5arwa/cursos.git HEAD f78a147444d202ce2f1468097593a8877b7500cc

### Classification report

PPCR: 0.855

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.996| 0.984| 0.965| 0.990| 0.980| 18792| 19162| 0.981 |
| `␣` | 0.959| 0.995| 0.934| 0.977| 0.947| 10123| 10782| 0.939 |
| `⏎␣⁺␣⁺` | 0.982| 0.967| 0.966| 0.975| 0.974| 1131| 1132| 0.999 |
| `⏎␣⁻␣⁻` | 0.997| 0.893| 0.822| 0.942| 0.901| 1033| 1122| 0.921 |
| `⏎` | 0.980| 0.889| 0.216| 0.932| 0.354| 434| 1786| 0.243 |
| `'` | 1.000| 1.000| 0.211| 1.000| 0.349| 382| 1807| 0.211 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1100| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 398| 0.000 |
| `macro avg` | 0.739| 0.716| 0.514| 0.727| 0.563| 31895| 37289| 0.855 |
| `micro avg` | 0.983| 0.983| 0.841| 0.983| 0.906| 31895| 37289| 0.855 |
| `weighted avg` | 0.984| 0.983| 0.841| 0.983| 0.868| 31895| 37289| 0.855 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|370 |18498 |294 |0 |0 |0 |0 |0 |0 |
|659 |18 |10074 |0 |8 |0 |20 |3 |0 |
|1425 |0 |0 |382 |0 |0 |0 |0 |0 |
|1352 |2 |46 |0 |386 |0 |0 |0 |0 |
|1100 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |37 |0 |0 |0 |1094 |0 |0 |
|89 |60 |51 |0 |0 |0 |0 |922 |0 |
|398 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Udemy/Angular/02-spa/src/assets/libs/bootstrap/js/bootstrap.bundle.js | 433 |
| Udemy/typeScript/typescript-importer/webpack.prod.js | 16 |
| Udemy/typeScript/typescript-importer/webpack.config.js | 10 |
| Udemy/Angular/03-pipes/karma.conf.js | 9 |
| Udemy/Angular/01-hola-mundo/karma.conf.js | 9 |
| Udemy/Angular/04-spotiapp/karma.conf.js | 9 |
| Udemy/Angular/02-spa/karma.conf.js | 9 |
| Udemy/typeScript/typescript/typescript/type/funcion_flecha.js | 8 |
| Udemy/Angular/02-spa/e2e/protractor.conf.js | 7 |
| Udemy/Angular/01-hola-mundo/e2e/protractor.conf.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 382}, "macro avg": {"f1-score": 0.726952427248754, "precision": 0.7391798195262135, "recall": 0.7160933843244478, "support": 31895}, "micro avg": {"f1-score": 0.9831007994983539, "precision": 0.9831007994983539, "recall": 0.9831007994983539, "support": 31895}, "weighted avg": {"f1-score": 0.9830568824194009, "precision": 0.9835101627305768, "recall": 0.9831007994983539, "support": 31895}, "\u2205": {"f1-score": 0.9899919721701901, "precision": 0.9956938314134999, "recall": 0.9843550446998723, "support": 18792}, "\u23ce": {"f1-score": 0.9323671497584543, "precision": 0.9796954314720813, "recall": 0.8894009216589862, "support": 434}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9746102449888641, "precision": 0.9820466786355476, "recall": 0.9672855879752431, "support": 1131}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9417773237997957, "precision": 0.9967567567567568, "recall": 0.8925459825750242, "support": 1033}, "\u2423": {"f1-score": 0.9768727272727273, "precision": 0.9592458579318225, "recall": 0.9951595376864566, "support": 10123}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 398}, "\u0027": {"f1-score": 0.3490178163544998, "precision": 1.0, "recall": 0.21140011068068623, "support": 1807}, "macro avg": {"f1-score": 0.5631332245344891, "precision": 0.7391798195262135, "recall": 0.514423324265469, "support": 37289}, "micro avg": {"f1-score": 0.9064523589269196, "precision": 0.9831007994983539, "recall": 0.8408914156989997, "support": 37289}, "weighted avg": {"f1-score": 0.8680152125202767, "precision": 0.9442153986342366, "recall": 0.8408914156989997, "support": 37289}, "\u2205": {"f1-score": 0.9802861685214626, "precision": 0.9956938314134999, "recall": 0.9653480847510698, "support": 19162}, "\u23ce": {"f1-score": 0.3541284403669725, "precision": 0.9796954314720813, "recall": 0.21612541993281076, "support": 1786}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1100}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9741763134461265, "precision": 0.9820466786355476, "recall": 0.9664310954063604, "support": 1132}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9008304836345873, "precision": 0.9967567567567568, "recall": 0.82174688057041, "support": 1122}, "\u2423": {"f1-score": 0.9466265739522645, "precision": 0.9592458579318225, "recall": 0.9343350027824151, "support": 10782}},
  "ppcr": 0.8553460806135857
}
```
</details>
