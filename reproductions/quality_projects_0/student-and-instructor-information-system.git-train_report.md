# Train report for javascript / file:///tmp/top-repos-quality-repos-1p5yybar/student-and-instructor-information-system.git HEAD b8a3ea744f01f038e2ca13dd6d45ea638d082051

### Classification report

PPCR: 0.785

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 0.996| 0.947| 0.984| 0.960| 21412| 22516| 0.951 |
| `␣` | 0.962| 0.957| 0.631| 0.960| 0.762| 4523| 6859| 0.659 |
| `"` | 0.971| 1.000| 0.749| 0.985| 0.846| 2075| 2769| 0.749 |
| `⏎` | 0.914| 0.794| 0.184| 0.850| 0.306| 349| 1507| 0.232 |
| `'` | 1.000| 0.819| 0.303| 0.900| 0.465| 342| 924| 0.370 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 269| 1018| 0.264 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 143| 1041| 0.137 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 342| 0.053 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 85| 0.153 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 57| 0.123 |
| `macro avg` | 0.482| 0.457| 0.281| 0.468| 0.334| 29151| 37118| 0.785 |
| `micro avg` | 0.970| 0.970| 0.762| 0.970| 0.854| 29151| 37118| 0.785 |
| `weighted avg` | 0.955| 0.970| 0.762| 0.963| 0.810| 29151| 37118| 0.785 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1104 |21326 |85 |0 |1 |0 |0 |0 |0 |0 |0 |
|2336 |168 |4330 |0 |25 |0 |0 |0 |0 |0 |0 |
|694 |0 |0 |2075 |0 |0 |0 |0 |0 |0 |0 |
|1158 |37 |35 |0 |277 |0 |0 |0 |0 |0 |0 |
|898 |104 |39 |0 |0 |0 |0 |0 |0 |0 |0 |
|749 |268 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|582 |0 |0 |62 |0 |0 |0 |280 |0 |0 |0 |
|324 |9 |9 |0 |0 |0 |0 |0 |0 |0 |0 |
|50 |5 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|72 |13 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/src/components/admin/FacultyManagement.js | 79 |
| client/src/components/admin/InstructorManagement.js | 75 |
| client/src/components/admin/CourseManagement.js | 73 |
| client/src/components/pages/Register.js | 50 |
| client/src/serviceWorker.js | 48 |
| client/src/components/admin/StudentManagement.js | 48 |
| client/src/components/Routes.js | 34 |
| client/src/components/admin/AdminManagement.js | 33 |
| client/src/components/functions/StyledDropzone.js | 30 |
| client/src/components/pages/sections/AssignmentAdd.js | 24 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9852801519468186, "precision": 0.970987365465606, "recall": 1.0, "support": 2075}, "\u0027": {"f1-score": 0.9003215434083601, "precision": 1.0, "recall": 0.8187134502923976, "support": 342}, "macro avg": {"f1-score": 0.46790381745118087, "precision": 0.48196450475132757, "recall": 0.4565722492263262, "support": 29151}, "micro avg": {"f1-score": 0.9703955267400776, "precision": 0.9703955267400776, "recall": 0.9703955267400776, "support": 29151}, "weighted avg": {"f1-score": 0.9625943801076959, "precision": 0.9553457042099197, "recall": 0.9703955267400776, "support": 29151}, "\u2205": {"f1-score": 0.984080107055512, "precision": 0.9724578203374373, "recall": 0.995983560620213, "support": 21412}, "\u23ce": {"f1-score": 0.8496932515337422, "precision": 0.9141914191419142, "recall": 0.7936962750716332, "support": 349}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 143}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 269}, "\u2423": {"f1-score": 0.959663120567376, "precision": 0.9620084425683182, "recall": 0.9573292062790183, "support": 4523}},
  "cl_report_full": {"\"": {"f1-score": 0.8459029759478189, "precision": 0.970987365465606, "recall": 0.7493680028891296, "support": 2769}, "\u0027": {"f1-score": 0.4651162790697675, "precision": 1.0, "recall": 0.30303030303030304, "support": 924}, "macro avg": {"f1-score": 0.3339056959471411, "precision": 0.48196450475132757, "recall": 0.28146432516928016, "support": 37118}, "micro avg": {"f1-score": 0.8537325144486864, "precision": 0.9703955267400776, "recall": 0.7621100274799288, "support": 37118}, "weighted avg": {"f1-score": 0.8100998099116854, "precision": 0.9021129013932582, "recall": 0.7621100274799288, "support": 37118}, "\u2205": {"f1-score": 0.9596364127255546, "precision": 0.9724578203374373, "recall": 0.9471486942618582, "support": 22516}, "\u23ce": {"f1-score": 0.3060773480662983, "precision": 0.9141914191419142, "recall": 0.1838088918380889, "support": 1507}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 342}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 57}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1041}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 85}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1018}, "\u2423": {"f1-score": 0.7623239436619719, "precision": 0.9620084425683182, "recall": 0.6312873596734218, "support": 6859}},
  "ppcr": 0.7853602025971227
}
```
</details>
