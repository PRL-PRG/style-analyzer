# Model report for file:///tmp/top-repos-quality-repos-jt8edkz7/gms.git HEAD 7fceb57ebaeb78e497572dc3686545aff2920365

### Dump

```json
{'created_at': '2021-08-21 03:23:08',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '28.3 kB',
 'tags': [],
 'uuid': '7094596a-1fa0-4752-a829-1cffa77e0ab7',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-jt8edkz7/gms.git 7fceb57ebaeb78e497572dc3686545aff2920365

# javascript
382 rules, avg.len. 9.5
## train
PPCR: 0.970289
### report
macro
{'f1-score': 0.46634286908259265,
 'precision': 0.5326737619450156,
 'recall': 0.43517558401910283,
 'support': 70475}
micro
{'f1-score': 0.906619368570415,
 'precision': 0.906619368570415,
 'recall': 0.906619368570415,
 'support': 70475}
weighted
{'f1-score': 0.8957442136674169,
 'precision': 0.8917978743490138,
 'recall': 0.906619368570415,
 'support': 70475}
### report_full
macro
{'f1-score': 0.43805347955205387,
 'precision': 0.5326737619450156,
 'recall': 0.40445739746910364,
 'support': 72633}
micro
{'f1-score': 0.8929479833412528,
 'precision': 0.906619368570415,
 'recall': 0.8796827888150014,
 'support': 72633}
weighted
{'f1-score': 0.8742105616398361,
 'precision': 0.8815870285129549,
 'recall': 0.8796827888150014,
 'support': 72633}
## test
PPCR: 0.942421
### report
macro
{'f1-score': 0.3109470189861139,
 'precision': 0.40279703294940555,
 'recall': 0.2949046970566063,
 'support': 17284}
micro
{'f1-score': 0.8278754917843092,
 'precision': 0.8278754917843092,
 'recall': 0.8278754917843092,
 'support': 17284}
weighted
{'f1-score': 0.8096889222602123,
 'precision': 0.8145721016596397,
 'recall': 0.8278754917843092,
 'support': 17284}
### report_full
macro
{'f1-score': 0.287359358525574,
 'precision': 0.40279703294940555,
 'recall': 0.2700021165772155,
 'support': 18340}
micro
{'f1-score': 0.8033348304513812,
 'precision': 0.8278754917843092,
 'recall': 0.7802071973827699,
 'support': 18340}
weighted
{'f1-score': 0.7679286086156483,
 'precision': 0.7911164213406852,
 'recall': 0.7802071973827699,
 'support': 18340}
```

## javascript
### Summary
255 rules, avg.len. 8.9

| | |
|-|-|
|Min support|134|
|Max support|11250|
|Min confidence|0.921444833278656|
|Max confidence|0.9998860359191895|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'max_features': 'auto',
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -3.roles in {KEY}<br>	∧ -4.reserved = '<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 331.` |
| 2 | `  -3.roles in {KEY}<br>	∧ -4.reserved not in {'}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.964. Support: 380.` |
| 3 | `  -3.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {MAP}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = "<br>Confidence: 0.975. Support: 3646.` |
| 4 | `  +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {LIST, MAP}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 2093.` |
| 5 | `  -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type = StringLiteral<br>	∧ ^1.roles in {MAP} and not in {LIST}<br>⇒ y = '<br>Confidence: 0.997. Support: 551.` |
| 6 | `  -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = [<br>	∧ ^1.roles in {MAP} and not in {LIST}<br>⇒ y = "<br>Confidence: 0.979. Support: 833.` |
| 7 | `  -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {[}<br>	∧ +3.roles in {STRING}<br>	∧ ^1.roles in {MAP} and not in {LIST}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 617.` |
| 8 | `  -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ -4.roles in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {[}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.roles in {MAP} and not in {LIST}<br>⇒ y = "<br>Confidence: 0.999. Support: 903.` |
| 9 | `  -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {[}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.roles in {MAP} and not in {LIST}<br>⇒ y = "<br>Confidence: 0.963. Support: 1779.` |
| 10 | `  -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {[}<br>	∧ ^1.roles in {MAP} and not in {LIST}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 2827.` |
| 11 | `  -1.roles in {MAP}<br>	∧ -5.diff_col ≥ 68<br>	∧ ^1.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.999. Support: 858.` |
| 12 | `  -1.reserved = {<br>	∧ -1.roles not in {MAP}<br>	∧ -5.diff_col ≥ 68<br>	∧ ^1.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.999. Support: 869.` |
| 13 | `  -5.diff_col ≤ 67<br>	∧ ^1.roles in {QUALIFIED} and not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 8518.` |
| 14 | `  -5.diff_col ≤ 67<br>	∧ +1.reserved = (<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 1564.` |
| 15 | `  -1.diff_col ≥ 3<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -3.diff_col ≥ 8<br>	∧ -5.diff_col ≤ 67<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 345.` |
| 16 | `  -5.diff_col ≤ 67<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 1387.` |
| 17 | `  -1.internal_type = Identifier<br>	∧ -3.label not in {"}<br>	∧ -5.diff_col ≤ 67<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {BinaryExpression, CallExpression}<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 2847.` |
| 18 | `  -1.internal_type not in {Identifier}<br>	∧ -2.diff_offset ≥ 25<br>	∧ -3.label not in {"}<br>	∧ -5.diff_col ≤ 67<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.965. Support: 215.` |
| 19 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -2.diff_offset ≤ 24<br>	∧ -3.label not in {"}<br>	∧ -5.diff_col ≤ 67<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 1610.` |
| 20 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved = =<br>	∧ -2.diff_offset ≤ 24<br>	∧ -3.label not in {"}<br>	∧ -5.diff_col ≤ 67<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 760.` |
| 21 | `  -1.internal_type not in {Identifier}<br>	∧ -2.reserved = )<br>	∧ -3.label not in {"}<br>	∧ -5.diff_col ≤ 67<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = new<br>	∧ ^1.internal_type not in {CallExpression, File}<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 412.` |
| 22 | `  -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {"}<br>	∧ -5.diff_col ≤ 67<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {LITERAL} and not in {MAP, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 1685.` |
| 23 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {"}<br>	∧ -3.reserved not in {(}<br>	∧ -5.diff_col ≤ 67<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {INSTANCE} and not in {LITERAL, MAP, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 1285.` |
| 24 | `  •••start_line ≤ 208<br>	∧ -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.internal_type = Identifier<br>	∧ -3.label not in {"}<br>	∧ -3.reserved not in {(}<br>	∧ -5.diff_col ≤ 67<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.roles in {RIGHT}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INSTANCE, LITERAL, MAP, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 208.` |
| 25 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {"}<br>	∧ -3.reserved not in {(}<br>	∧ -5.diff_col ≤ 67<br>	∧ -5.reserved = var<br>	∧ +1.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {EXPRESSION, INSTANCE, LITERAL, MAP, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 234.` |
| 26 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {"}<br>	∧ -3.reserved not in {(}<br>	∧ -5.diff_col ≤ 67<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {EXPRESSION, INSTANCE, LITERAL, MAP, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 434.` |
| 27 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {"}<br>	∧ -3.reserved not in {(}<br>	∧ -5.diff_col ≤ 67<br>	∧ -5.reserved not in {var}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {RIGHT}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {EXPRESSION, INSTANCE, LITERAL, MAP, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 759.` |
| 28 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {"}<br>	∧ -3.reserved not in {(}<br>	∧ -5.diff_col ≤ 67<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {), {}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {EXPRESSION, INSTANCE, LITERAL, MAP, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 290.` |
| 29 | `  -1.roles in {STRING}<br>	∧ -4.internal_type = NumericLiteral<br>	∧ -5.diff_col ≥ 19<br>⇒ y = "<br>Confidence: 0.999. Support: 861.` |
| 30 | `  -1.roles in {STRING}<br>	∧ -4.internal_type not in {NumericLiteral}<br>	∧ -5.diff_col ≥ 28<br>	∧ +2.reserved = [<br>⇒ y = "<br>Confidence: 0.984. Support: 866.` |
| 31 | `  -1.roles in {STRING}<br>	∧ -3.diff_offset ≥ 23<br>	∧ -4.internal_type not in {NumericLiteral}<br>	∧ -5.diff_col ≥ 28<br>	∧ +2.reserved not in {[}<br>⇒ y = "<br>Confidence: 0.955. Support: 2743.` |
| 32 | `  -1.diff_col ≥ 4<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<+space>}<br>	∧ +2.roles in {RIGHT}<br>	∧ ^1.roles in {BINARY} and not in {FILE}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 473.` |
| 33 | `  -1.diff_col ≥ 4<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<+space>}<br>	∧ +2.roles not in {RIGHT}<br>	∧ ^1.roles in {BINARY, QUALIFIED} and not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 631.` |
| 34 | `  -1.diff_col ≥ 4<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<+space>}<br>	∧ +2.roles not in {RIGHT}<br>	∧ ^1.roles in {BINARY, FUNCTION} and not in {FILE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 134.` |
| 35 | `  -1.diff_col ≥ 4<br>	∧ -1.roles not in {STRING}<br>	∧ -5.diff_line ≥ 1<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {BINARY, FILE}<br>	∧ ^2.roles not in {FUNCTION, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 1796.` |
| 36 | `  -1.diff_col ≥ 4<br>	∧ -1.roles not in {STRING}<br>	∧ -5.diff_line = 0<br>	∧ ^1.roles not in {BINARY, FILE}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 7334.` |
| 37 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_col ≥ 7<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.roles in {STRING}<br>⇒ y = "<br>Confidence: 0.993. Support: 3616.` |
| 38 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_col ≤ 6<br>	∧ -5.roles in {LITERAL}<br>	∧ +1.roles in {STRING}<br>⇒ y = "<br>Confidence: 0.949. Support: 617.` |
| 39 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_col ≤ 6<br>	∧ -3.internal_type = NumericLiteral<br>	∧ -5.roles not in {LITERAL}<br>	∧ +1.roles in {STRING}<br>⇒ y = "<br>Confidence: 0.980. Support: 275.` |
| 40 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_col ≤ 6<br>	∧ -3.internal_type not in {NumericLiteral}<br>	∧ -4.reserved = '<br>	∧ -5.roles not in {LITERAL}<br>	∧ +1.roles in {STRING}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = ␣<br>Confidence: 0.999. Support: 345.` |
| 41 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_col ≤ 6<br>	∧ -3.internal_type not in {NumericLiteral}<br>	∧ -4.reserved not in {'}<br>	∧ -5.roles not in {LITERAL}<br>	∧ +1.roles in {STRING}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = '<br>Confidence: 0.938. Support: 393.` |
| 42 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_col ≤ 6<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.internal_type not in {NumericLiteral}<br>	∧ -5.reserved = :<br>	∧ -5.roles not in {LITERAL}<br>	∧ +1.roles in {STRING}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 281.` |
| 43 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.length ≥ 10<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 445.` |
| 44 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = NumericLiteral<br>	∧ +1.roles not in {STRING}<br>	∧ +2.length ≤ 9<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 1652.` |
| 45 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {NumericLiteral}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.length ≤ 9<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 156.` |
| 46 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {NumericLiteral}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.length ≤ 9<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 2000.` |
| 47 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -5.diff_col ≥ 60<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {LITERAL}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.978. Support: 851.` |
| 48 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -5.diff_col ≤ 59<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {LITERAL}<br>	∧ ^1.roles in {MAP}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 304.` |
| 49 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 5072.` |
| 50 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≥ 3<br>	∧ -3.reserved not in {)}<br>	∧ -5.roles not in {EXPRESSION}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {LITERAL}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FUNCTION} and not in {CALL}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 156.` |
| 51 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -3.roles in {LEFT}<br>	∧ -5.roles not in {EXPRESSION}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {LITERAL}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CALL, FUNCTION}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 278.` |
| 52 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -3.roles in {IDENTIFIER} and not in {LEFT}<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ -5.roles not in {EXPRESSION}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {LITERAL}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CALL, FUNCTION}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 389.` |
| 53 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -4.reserved not in {-}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1768.` |
| 54 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -4.reserved not in {-}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1711.` |
| 55 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -4.reserved not in {-}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {,, ;}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.roles in {LITERAL}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 915.` |
| 56 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -4.diff_line ≥ 1<br>	∧ -4.reserved not in {-}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {,, ;}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 6<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {LITERAL}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 624.` |
| 57 | `  -1.diff_col ≤ 3<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -4.diff_line = 0<br>	∧ -4.reserved not in {-}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {,, ;}<br>	∧ +1.roles not in {LITERAL, STRING}<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {BlockStatement, MemberExpression}<br>	∧ ^1.roles in {BINARY} and not in {LITERAL}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 235.` |
| 58 | `  -1.diff_col ≤ 3<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {)}<br>	∧ -4.diff_line = 0<br>	∧ -4.reserved not in {-}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {BINARY, FUNCTION, LITERAL}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 519.` |
| 59 | `  -3.diff_offset ≥ 9<br>	∧ +1.reserved = [<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = ∅<br>Confidence: 0.991. Support: 868.` |
| 60 | `  •••start_col ≥ 40<br>	∧ -3.diff_offset ≥ 9<br>	∧ +1.reserved not in {[}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = "<br>Confidence: 0.991. Support: 3411.` |
| 61 | `  •••start_col ≤ 39<br>	∧ -3.diff_offset ≥ 9<br>	∧ -5.diff_col ≥ 107<br>	∧ +1.reserved not in {[}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = "<br>Confidence: 0.999. Support: 860.` |
| 62 | `  -3.diff_offset ≤ 8<br>	∧ -4.diff_col ≥ 5<br>	∧ +3.internal_type = StringLiteral<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = '<br>Confidence: 0.980. Support: 329.` |
| 63 | `  -3.diff_offset ≤ 8<br>	∧ -4.diff_col ≤ 5<br>	∧ +3.internal_type = StringLiteral<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = ␣<br>Confidence: 0.999. Support: 335.` |
| 64 | `  -3.diff_offset ≤ 8<br>	∧ -4.roles in {KEY}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = ∅<br>Confidence: 0.995. Support: 872.` |
| 65 | `  -5.diff_offset ≥ 28<br>	∧ -5.roles in {VALUE}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = "<br>Confidence: 0.994. Support: 1784.` |
| 66 | `  -5.diff_offset ≥ 28<br>	∧ -5.roles not in {VALUE}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.984. Support: 1755.` |
| 67 | `  -5.diff_offset ≥ 28<br>	∧ -5.roles not in {VALUE}<br>	∧ +5.roles in {LITERAL}<br>	∧ ^1.roles in {LITERAL}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.952. Support: 827.` |
| 68 | `  -5.diff_offset ≥ 28<br>	∧ -5.roles not in {VALUE}<br>	∧ +5.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 148.` |
| 69 | `  -5.diff_offset ≥ 28<br>	∧ -5.reserved = :<br>	∧ -5.roles not in {VALUE}<br>	∧ +5.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression, VariableDeclaration}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 912.` |
| 70 | `  -5.diff_offset ≥ 28<br>	∧ -5.reserved not in {:}<br>	∧ -5.roles not in {VALUE}<br>	∧ +1.reserved not in {,}<br>	∧ +3.length ≤ 1<br>	∧ +5.roles not in {LITERAL}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression, VariableDeclaration}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 754.` |
| 71 | `  -1.internal_type = NumericLiteral<br>	∧ -5.diff_offset ≤ 27<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 3048.` |
| 72 | `  -1.internal_type not in {NumericLiteral}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -5.diff_offset ≤ 27<br>	∧ +5.roles in {MAP}<br>	∧ ^2.internal_type = ArrayExpression<br>⇒ y = "<br>Confidence: 0.987. Support: 913.` |
| 73 | `  -1.internal_type not in {NumericLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {LITERAL}<br>	∧ -5.diff_offset ≤ 27<br>	∧ +5.roles not in {MAP}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 1260.` |
| 74 | `  -1.internal_type not in {NumericLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.label not in {<space>}<br>	∧ -5.diff_offset ≤ 27<br>	∧ +1.roles in {RIGHT}<br>	∧ +5.roles not in {MAP}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 572.` |
| 75 | `  -1.internal_type not in {NumericLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -5.diff_offset ≤ 27<br>	∧ +1.reserved = -<br>	∧ +5.roles not in {MAP}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 481.` |
| 76 | `  -1.internal_type not in {NumericLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -5.diff_offset ≤ 27<br>	∧ +1.roles not in {RIGHT}<br>	∧ +5.roles not in {MAP}<br>	∧ ^1.roles in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 7349.` |
| 77 | `  -1.internal_type not in {NumericLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -5.diff_offset ≤ 27<br>	∧ +2.roles in {RIGHT}<br>	∧ +5.roles not in {MAP}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 668.` |
| 78 | `  -1.internal_type not in {NumericLiteral}<br>	∧ -1.reserved not in {;, =}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.diff_col ≤ 17<br>	∧ -5.diff_offset ≤ 27<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {RIGHT}<br>	∧ +5.roles not in {MAP}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 920.` |
| 79 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {;, =}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.diff_col ≤ 17<br>	∧ -3.reserved = )<br>	∧ -5.diff_offset ≤ 27<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +5.roles not in {MAP}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 147.` |
| 80 | `  -1.internal_type not in {NumericLiteral}<br>	∧ -1.reserved not in {;, =}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.diff_col ≤ 17<br>	∧ -3.reserved not in {)}<br>	∧ -5.diff_offset ≤ 27<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {RIGHT}<br>	∧ +5.roles not in {MAP}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2042.` |
| 81 | `  -1.internal_type not in {NumericLiteral}<br>	∧ -1.reserved not in {;, =}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.diff_col ≤ 17<br>	∧ -3.reserved not in {)}<br>	∧ -5.diff_offset ≤ 27<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {RIGHT}<br>	∧ +5.roles not in {MAP}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1752.` |
| 82 | `  -1.internal_type not in {NumericLiteral}<br>	∧ -1.reserved not in {;, =}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.diff_col ≤ 17<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -3.reserved not in {)}<br>	∧ -3.roles not in {LITERAL}<br>	∧ -4.diff_line = 0<br>	∧ -4.reserved not in {,}<br>	∧ -5.diff_offset ≤ 27<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {,, ;, {}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +5.roles not in {MAP}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 257.` |
| 83 | `  -1.internal_type not in {NumericLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.diff_col ≤ 17<br>	∧ -3.reserved not in {)}<br>	∧ -5.diff_offset ≤ 27<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {RIGHT}<br>	∧ +5.roles not in {MAP}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 477.` |
| 84 | `  -1.internal_type not in {NumericLiteral}<br>	∧ -1.reserved not in {;, =}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.diff_col ≤ 17<br>	∧ -3.reserved not in {)}<br>	∧ -4.diff_offset ≥ 13<br>	∧ -5.diff_offset ≤ 27<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved not in {,, -, ;, =, {}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +5.roles not in {MAP}<br>	∧ ^1.roles in {LIST} and not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 444.` |
| 85 | `  ^1.roles in {LITERAL, VALUE}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 2194.` |
| 86 | `  -3.label in {"}<br>	∧ ^1.roles in {LITERAL} and not in {VALUE}<br>⇒ y = ⏎<br>Confidence: 0.989. Support: 900.` |
| 87 | `  -2.internal_type = NumericLiteral<br>	∧ -3.label not in {"}<br>	∧ ^1.roles in {LITERAL} and not in {VALUE}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 1260.` |
| 88 | `  -2.internal_type not in {NumericLiteral}<br>	∧ -2.label not in {'}<br>	∧ -3.label not in {"}<br>	∧ +2.reserved = :<br>	∧ ^1.internal_type = ArrayExpression<br>	∧ ^1.roles in {LITERAL} and not in {VALUE}<br>⇒ y = "<br>Confidence: 0.999. Support: 941.` |
| 89 | `  -2.internal_type not in {NumericLiteral}<br>	∧ -2.label not in {'}<br>	∧ -3.label not in {"}<br>	∧ -5.reserved = ,<br>	∧ +2.reserved = :<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles in {LITERAL} and not in {VALUE}<br>⇒ y = "<br>Confidence: 1.000. Support: 1128.` |
| 90 | `  -2.internal_type not in {NumericLiteral}<br>	∧ -3.label not in {"}<br>	∧ -5.roles in {NUMBER}<br>	∧ +2.reserved not in {:}<br>	∧ +3.roles in {EXPRESSION}<br>	∧ ^1.roles in {LITERAL} and not in {VALUE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 428.` |
| 91 | `  -2.internal_type not in {NumericLiteral}<br>	∧ -3.label not in {"}<br>	∧ -5.roles not in {NUMBER}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {:}<br>	∧ +3.roles in {EXPRESSION}<br>	∧ ^1.roles in {LITERAL} and not in {VALUE}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 1854.` |
| 92 | `  -2.internal_type not in {NumericLiteral}<br>	∧ -3.label not in {"}<br>	∧ -4.roles in {STRING}<br>	∧ +2.reserved not in {:}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {LITERAL} and not in {VALUE}<br>⇒ y = '<br>Confidence: 0.924. Support: 415.` |
| 93 | `  -2.internal_type not in {NumericLiteral}<br>	∧ -3.label not in {"}<br>	∧ -3.roles in {KEY}<br>	∧ -4.roles not in {STRING}<br>	∧ +2.reserved not in {:}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {LITERAL} and not in {VALUE}<br>⇒ y = "<br>Confidence: 1.000. Support: 1778.` |
| 94 | `  -3.length ≥ 15<br>	∧ +2.roles in {MAP}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = "<br>Confidence: 0.982. Support: 866.` |
| 95 | `  -3.reserved = ]<br>	∧ -3.length ≤ 14<br>	∧ +2.roles in {MAP}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = "<br>Confidence: 0.999. Support: 856.` |
| 96 | `  -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {]}<br>	∧ -3.length ≤ 14<br>	∧ +2.roles in {MAP}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = '<br>Confidence: 0.957. Support: 243.` |
| 97 | `  -2.roles in {LITERAL}<br>	∧ -5.diff_col ≥ 19<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = "<br>Confidence: 0.958. Support: 1858.` |
| 98 | `  -2.roles not in {LITERAL}<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.roles not in {LITERAL}<br>	∧ ^2.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.940. Support: 2738.` |
| 99 | `  -2.roles not in {LITERAL}<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.roles in {QUALIFIED} and not in {LITERAL}<br>	∧ ^2.internal_type not in {MemberExpression, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 6375.` |
| 100 | `  -2.roles not in {LITERAL}<br>	∧ -3.internal_type = Identifier<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.roles in {BINARY} and not in {LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 1325.` |
| 101 | `  -2.roles not in {LITERAL}<br>	∧ -3.roles in {LEFT}<br>	∧ +2.roles not in {CALL, MAP}<br>	∧ ^1.roles in {BINARY} and not in {LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 202.` |
| 102 | `  -2.roles not in {LITERAL}<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.internal_type = NewExpression<br>	∧ ^1.roles not in {BINARY, LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 1768.` |
| 103 | `  -2.roles not in {LITERAL}<br>	∧ -3.label in {"}<br>	∧ +1.reserved not in {:}<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.internal_type not in {NewExpression}<br>	∧ ^1.roles not in {BINARY, LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 896.` |
| 104 | `  -2.roles not in {LITERAL}<br>	∧ -3.internal_type = StringLiteral<br>	∧ -3.label not in {"}<br>	∧ -5.reserved = ,<br>	∧ +1.reserved not in {:}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.internal_type not in {NewExpression}<br>	∧ ^1.roles not in {BINARY, LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {MemberExpression, VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.999. Support: 381.` |
| 105 | `  -1.reserved = (<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -3.label not in {"}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {:}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.internal_type not in {CallExpression, File, IfStatement, NewExpression}<br>	∧ ^1.roles not in {BINARY, LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 294.` |
| 106 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {{}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.label not in {"}<br>	∧ +1.reserved not in {:}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {NUMBER} and not in {MAP}<br>	∧ ^1.internal_type not in {NewExpression}<br>	∧ ^1.roles not in {BINARY, LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 813.` |
| 107 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.label not in {"}<br>	∧ +1.reserved not in {:}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {MAP, NUMBER}<br>	∧ ^1.roles not in {BINARY, LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 518.` |
| 108 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.label not in {"}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {MAP, NUMBER}<br>	∧ ^1.internal_type not in {NewExpression}<br>	∧ ^1.roles in {CALL} and not in {BINARY, LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 134.` |
| 109 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.label not in {"}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {MAP, NUMBER}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {BINARY, CALL, LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 182.` |
| 110 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 8616.` |
| 111 | `  -1.roles in {VALUE}<br>	∧ -2.diff_col ≥ 12<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.986. Support: 1759.` |
| 112 | `  -1.roles in {KEY} and not in {VALUE}<br>	∧ -1.length ≥ 5<br>	∧ -3.length ≥ 9<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.973. Support: 847.` |
| 113 | `  -1.roles in {KEY} and not in {VALUE}<br>	∧ -1.length ≥ 5<br>	∧ -3.length ≤ 8<br>	∧ -4.roles in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.970. Support: 961.` |
| 114 | `  -1.roles in {KEY} and not in {VALUE}<br>	∧ -1.length ≥ 5<br>	∧ -3.length ≤ 8<br>	∧ -4.diff_col ≥ 117<br>	∧ -4.roles not in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.999. Support: 791.` |
| 115 | `  -1.roles not in {KEY, VALUE}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.996. Support: 2667.` |
| 116 | `  •••start_col ≥ 3<br>	∧ -1.reserved = ,<br>	∧ -2.reserved not in {'}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +5.roles in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.999. Support: 850.` |
| 117 | `  •••start_col ≤ 2<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.999. Support: 902.` |
| 118 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {KEY, VALUE}<br>	∧ -1.length ≥ 11<br>	∧ +1.internal_type = Identifier<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.987. Support: 186.` |
| 119 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {KEY, VALUE}<br>	∧ -1.length ≤ 10<br>	∧ +1.internal_type = Identifier<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 1745.` |
| 120 | `  -1.reserved = (<br>	∧ -1.roles not in {KEY, VALUE}<br>	∧ -1.length ≤ 1<br>	∧ -2.diff_col ≥ 9<br>	∧ -4.reserved not in {,}<br>	∧ +1.internal_type = Identifier<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 673.` |
| 121 | `  -1.reserved = (<br>	∧ -1.roles not in {KEY, VALUE}<br>	∧ -1.length ≤ 1<br>	∧ -2.diff_col ≤ 8<br>	∧ +1.internal_type = Identifier<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 689.` |
| 122 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY, VALUE}<br>	∧ -1.length ≤ 1<br>	∧ -2.diff_col ≤ 8<br>	∧ -2.roles in {CALL}<br>	∧ +1.internal_type = Identifier<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 185.` |
| 123 | `  -1.roles not in {KEY, VALUE}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.roles in {RIGHT}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 262.` |
| 124 | `  -1.roles not in {KEY, VALUE}<br>	∧ -4.length ≥ 50<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 991.` |
| 125 | `  -1.roles not in {KEY, VALUE}<br>	∧ -4.length ≤ 49<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 877.` |
| 126 | `  -1.roles not in {KEY, VALUE}<br>	∧ -2.diff_col ≤ 16<br>	∧ -3.reserved = var<br>	∧ -4.length ≤ 49<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 651.` |
| 127 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {KEY, VALUE}<br>	∧ -2.diff_col ≤ 16<br>	∧ -3.reserved not in {), var}<br>	∧ -4.length ≤ 49<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 4388.` |
| 128 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {KEY, VALUE}<br>	∧ -2.diff_col ≤ 16<br>	∧ -2.reserved = )<br>	∧ -3.reserved not in {), var}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved not in {,, {}<br>	∧ +1.length ≥ 3<br>	∧ +5.length ≤ 1<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.961. Support: 444.` |
| 129 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {KEY, VALUE}<br>	∧ -2.diff_col ≤ 16<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved not in {), var}<br>	∧ -4.length ≤ 49<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved not in {,, -, {, }}<br>	∧ +2.internal_type not in {CommentLine, Identifier}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 648.` |
| 130 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {KEY, VALUE}<br>	∧ -2.diff_col ≤ 16<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved not in {), var}<br>	∧ -4.length ≤ 49<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved not in {,, -, {, }}<br>	∧ +2.internal_type not in {CommentLine, Identifier}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 9586.` |
| 131 | `  -4.diff_offset ≥ 25<br>	∧ -4.reserved = "<br>⇒ y = ∅<br>Confidence: 0.994. Support: 946.` |
| 132 | `  -4.diff_offset ≥ 25<br>	∧ -4.reserved not in {"}<br>	∧ +1.reserved = {<br>⇒ y = ⏎<br>Confidence: 0.973. Support: 840.` |
| 133 | `  -4.diff_offset ≥ 25<br>	∧ -4.reserved not in {"}<br>	∧ +1.reserved = =<br>	∧ +4.roles in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 154.` |
| 134 | `  -4.diff_offset ≥ 25<br>	∧ -4.reserved not in {"}<br>	∧ +1.reserved not in {{}<br>	∧ +4.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.946. Support: 2785.` |
| 135 | `  -4.diff_offset ≥ 25<br>	∧ -4.reserved not in {"}<br>	∧ -4.roles in {MAP}<br>	∧ +1.reserved not in {{}<br>	∧ +4.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = "<br>Confidence: 0.964. Support: 898.` |
| 136 | `  -4.diff_offset ≥ 25<br>	∧ -4.reserved not in {"}<br>	∧ -4.roles not in {MAP}<br>	∧ -5.roles in {KEY}<br>	∧ +1.reserved not in {{}<br>	∧ +4.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = "<br>Confidence: 0.997. Support: 836.` |
| 137 | `  -1.diff_col ≤ 18<br>	∧ -4.diff_offset ≥ 25<br>	∧ -4.reserved not in {"}<br>	∧ -4.roles not in {MAP}<br>	∧ -5.roles not in {KEY}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 12<br>	∧ +4.reserved not in {.}<br>	∧ +4.roles not in {IDENTIFIER}<br>	∧ +5.length ≤ 1<br>	∧ ^1.roles in {IDENTIFIER} and not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 261.` |
| 138 | `  -4.diff_offset ≤ 24<br>	∧ -5.roles in {LITERAL}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.966. Support: 2557.` |
| 139 | `  -3.roles in {NUMBER}<br>	∧ -4.diff_offset ≤ 24<br>	∧ -5.roles not in {LITERAL}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.986. Support: 322.` |
| 140 | `  -4.diff_offset ≤ 24<br>	∧ -4.internal_type = NumericLiteral<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {VALUE}<br>⇒ y = "<br>Confidence: 0.998. Support: 883.` |
| 141 | `  -1.reserved = =<br>	∧ -4.diff_offset ≤ 24<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles not in {VALUE}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 1244.` |
| 142 | `  -1.reserved not in {=}<br>	∧ -1.length ≤ 1<br>	∧ -4.diff_offset ≤ 24<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {BINARY}<br>	∧ +2.roles not in {EXPRESSION, VALUE}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 234.` |
| 143 | `  -1.reserved not in {=}<br>	∧ -2.reserved = )<br>	∧ -4.diff_offset ≤ 24<br>	∧ -5.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.928. Support: 480.` |
| 144 | `  -1.reserved not in {;, =}<br>	∧ -1.roles in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -4.diff_offset ≤ 24<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles not in {VALUE}<br>	∧ +3.internal_type = NumericLiteral<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.999. Support: 776.` |
| 145 | `  -1.reserved not in {;, =}<br>	∧ -1.roles in {STRING} and not in {CALL}<br>	∧ -2.reserved not in {)}<br>	∧ -4.diff_offset ≤ 24<br>	∧ -5.length ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {MAP} and not in {VALUE}<br>	∧ +3.internal_type not in {NumericLiteral}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.998. Support: 262.` |
| 146 | `  -1.reserved not in {;, =}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -4.diff_offset ≤ 24<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 655.` |
| 147 | `  -1.reserved not in {;, =}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -4.diff_offset ≤ 24<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 7403.` |
| 148 | `  -1.reserved not in {;, =}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.reserved = var<br>	∧ -4.diff_offset ≤ 24<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 470.` |
| 149 | `  -1.reserved not in {;, =}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<+space>}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 24<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 205.` |
| 150 | `  -1.reserved not in {;, =}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_line = 0<br>	∧ -4.diff_offset ≤ 24<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {{}<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 219.` |
| 151 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_line = 0<br>	∧ -4.diff_offset ≤ 24<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {{}<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 771.` |
| 152 | `  -1.reserved not in {;, =}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_offset ≥ 12<br>	∧ -4.diff_line = 0<br>	∧ -4.diff_offset ≤ 24<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 3963.` |
| 153 | `  -1.reserved not in {;, =}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_offset ≤ 11<br>	∧ -4.diff_line = 0<br>	∧ -4.diff_offset ≤ 24<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +2.roles not in {VALUE}<br>	∧ +2.length ≤ 9<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 9760.` |
| 154 | `  -1.internal_type = StringLiteral<br>	∧ -3.diff_col ≥ 10<br>	∧ -4.diff_col ≥ 18<br>⇒ y = "<br>Confidence: 0.921. Support: 4831.` |
| 155 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.994. Support: 1406.` |
| 156 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -4.reserved = ,<br>	∧ +1.roles in {LITERAL}<br>	∧ +3.length ≥ 15<br>⇒ y = "<br>Confidence: 0.994. Support: 612.` |
| 157 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -4.reserved not in {,}<br>	∧ +1.roles in {KEY, LITERAL}<br>	∧ +3.length ≥ 15<br>⇒ y = "<br>Confidence: 0.998. Support: 1210.` |
| 158 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ +1.roles in {LITERAL}<br>	∧ +3.length ≤ 14<br>	∧ ^2.internal_type = ArrayExpression<br>⇒ y = ∅<br>Confidence: 0.943. Support: 887.` |
| 159 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -2.roles in {STRING}<br>	∧ +1.roles in {LITERAL}<br>	∧ +3.length ≤ 14<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = "<br>Confidence: 0.990. Support: 1758.` |
| 160 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.roles not in {STRING}<br>	∧ -3.roles in {MAP}<br>	∧ +1.roles in {LITERAL}<br>	∧ +3.length ≤ 14<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 343.` |
| 161 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -2.roles not in {STRING}<br>	∧ -3.roles not in {MAP}<br>	∧ +1.roles in {LITERAL}<br>	∧ +3.length ≤ 14<br>	∧ ^1.roles in {MAP}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 1791.` |
| 162 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -2.roles not in {STRING}<br>	∧ -3.roles not in {MAP}<br>	∧ +1.roles in {LITERAL}<br>	∧ +3.length ≤ 14<br>	∧ +4.internal_type = NumericLiteral<br>	∧ ^1.roles not in {MAP}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = "<br>Confidence: 0.967. Support: 815.` |
| 163 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles in {LEFT}<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 446.` |
| 164 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 213.` |
| 165 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, =}<br>	∧ -1.roles in {FUNCTION} and not in {COMMENT, LEFT}<br>	∧ -1.length ≥ 3<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION, EXPRESSION}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 460.` |
| 166 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, =}<br>	∧ -1.length ≥ 3<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION, EXPRESSION}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 143.` |
| 167 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, =}<br>	∧ -1.length ≤ 2<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION, EXPRESSION}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 333.` |
| 168 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.roles not in {LITERAL}<br>	∧ +5.reserved = new<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 424.` |
| 169 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = AssignmentExpression<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 355.` |
| 170 | `  -1.diff_offset ≥ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -5.diff_line = 0<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = AssignmentExpression<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 176.` |
| 171 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = new<br>	∧ -2.diff_col ≥ 19<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 399.` |
| 172 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles not in {COMMENT, LEFT}<br>	∧ -2.diff_col ≤ 18<br>	∧ -2.diff_offset ≥ 3<br>	∧ -3.diff_col ≥ 6<br>	∧ +1.roles not in {LITERAL}<br>	∧ +5.reserved not in {new}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles in {EXPRESSION} and not in {BOOLEAN, DECLARATION}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 1848.` |
| 173 | `  •••start_col ≥ 30<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {COMMENT, LEFT}<br>	∧ -2.diff_col ≤ 18<br>	∧ -2.diff_offset ≥ 3<br>	∧ -3.diff_col ≥ 6<br>	∧ -5.diff_offset ≤ 22<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 4<br>	∧ +5.reserved not in {new}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles in {EXPRESSION} and not in {BOOLEAN, DECLARATION}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 249.` |
| 174 | `  •••start_col ≥ 30<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -2.diff_col ≤ 18<br>	∧ -2.diff_offset ≥ 3<br>	∧ -3.diff_col ≥ 6<br>	∧ -5.diff_offset ≤ 22<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 4<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 163.` |
| 175 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles not in {COMMENT, LEFT}<br>	∧ -2.diff_col ≤ 18<br>	∧ -2.diff_offset ≥ 3<br>	∧ -3.diff_col ≥ 6<br>	∧ +1.roles not in {LITERAL}<br>	∧ +5.reserved not in {new}<br>	∧ ^1.internal_type not in {AssignmentExpression, CallExpression}<br>	∧ ^1.roles in {EXPRESSION} and not in {BOOLEAN, DECLARATION}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 11250.` |
| 176 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -3.label in {"}<br>	∧ -5.diff_col ≥ 48<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 865.` |
| 177 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.reserved = )<br>	∧ -3.label not in {"}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.926. Support: 468.` |
| 178 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), =}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.reserved = )<br>	∧ -3.label not in {"}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {QUALIFIED} and not in {LITERAL}<br>	∧ ^2.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 194.` |
| 179 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, =}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_offset ≥ 10<br>	∧ -3.label not in {"}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 2342.` |
| 180 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, =}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_offset ≤ 9<br>	∧ -3.label not in {"}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {QUALIFIED} and not in {CALL}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 187.` |
| 181 | `  -2.roles in {STRING}<br>	∧ -5.diff_offset ≥ 20<br>⇒ y = "<br>Confidence: 0.978. Support: 2615.` |
| 182 | `  -1.diff_col ≥ 5<br>	∧ -2.roles not in {STRING}<br>	∧ +2.roles in {MAP}<br>⇒ y = "<br>Confidence: 0.929. Support: 2879.` |
| 183 | `  -1.diff_col ≤ 4<br>	∧ -2.roles not in {STRING}<br>	∧ -2.length ≤ 1<br>	∧ -4.diff_col ≤ 68<br>	∧ +2.roles in {MAP}<br>	∧ +5.roles in {NUMBER}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 748.` |
| 184 | `  -1.diff_col ≤ 1<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -4.diff_col ≤ 68<br>	∧ -4.diff_offset ≤ 21<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.internal_type = BinaryExpression<br>⇒ y = ␣<br>Confidence: 0.972. Support: 197.` |
| 185 | `  -1.reserved = =<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 1402.` |
| 186 | `  -1.reserved not in {=}<br>	∧ -2.roles not in {STRING}<br>	∧ -3.internal_type = NumericLiteral<br>	∧ +1.roles in {KEY, LITERAL}<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>⇒ y = "<br>Confidence: 0.999. Support: 906.` |
| 187 | `  -1.reserved not in {=}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {MAP}<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^2.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.978. Support: 996.` |
| 188 | `  -1.reserved not in {=}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {MAP}<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^2.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 418.` |
| 189 | `  -1.reserved = new<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {MAP}<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^2.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 497.` |
| 190 | `  -1.reserved not in {=, new}<br>	∧ -2.diff_col ≤ 19<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {MAP}<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^2.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 1090.` |
| 191 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = ,<br>	∧ -1.length ≤ 2<br>	∧ -2.diff_col ≤ 19<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ -5.reserved not in {.}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {,}<br>	∧ +2.roles not in {MAP}<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 152.` |
| 192 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles in {CALL}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {MAP}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1090.` |
| 193 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles not in {CALL}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {MAP}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 273.` |
| 194 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles not in {CALL}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {MAP}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 144.` |
| 195 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles not in {CALL}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {MAP}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type = AssignmentExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 688.` |
| 196 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles not in {CALL}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {MAP}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {AssignmentExpression, BinaryExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 6212.` |
| 197 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {=}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ -5.internal_type not in {Identifier}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {MAP}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 186.` |
| 198 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {=}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {MAP}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 390.` |
| 199 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = if<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {MAP}<br>	∧ +3.reserved not in {)}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 157.` |
| 200 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, =, if}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_col ≤ 68<br>	∧ -5.diff_line = 0<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {COMMENT, MAP}<br>	∧ +3.reserved not in {)}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {BinaryExpression, FunctionExpression}<br>	∧ ^1.roles not in {SCOPE}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 9863.` |
| 201 | `  -1.internal_type = StringLiteral<br>	∧ -4.roles in {VALUE}<br>⇒ y = '<br>Confidence: 0.998. Support: 290.` |
| 202 | `  -1.internal_type = StringLiteral<br>	∧ -4.reserved = ,<br>	∧ -4.roles not in {VALUE}<br>	∧ -5.internal_type not in {Identifier}<br>⇒ y = "<br>Confidence: 0.999. Support: 2544.` |
| 203 | `  -1.internal_type = StringLiteral<br>	∧ -2.diff_col ≤ 9<br>	∧ -4.reserved not in {,}<br>	∧ -4.roles not in {VALUE}<br>	∧ -5.diff_col ≥ 17<br>	∧ -5.internal_type not in {Identifier}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.983. Support: 1779.` |
| 204 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_col ≥ 7<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = "<br>Confidence: 0.975. Support: 2736.` |
| 205 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_col ≥ 122<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = "<br>Confidence: 0.999. Support: 889.` |
| 206 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_col ≤ 6<br>	∧ -5.length ≥ 8<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {KEY}<br>⇒ y = "<br>Confidence: 0.971. Support: 636.` |
| 207 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_col ≤ 6<br>	∧ -4.label not in {'}<br>	∧ -5.length ≤ 7<br>	∧ +1.internal_type = StringLiteral<br>	∧ +5.roles in {VALUE}<br>⇒ y = '<br>Confidence: 0.973. Support: 356.` |
| 208 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_col ≤ 6<br>	∧ -2.label not in {<space>}<br>	∧ -3.internal_type = NumericLiteral<br>	∧ -4.label not in {'}<br>	∧ -5.length ≤ 7<br>	∧ +1.internal_type = StringLiteral<br>	∧ +5.roles not in {VALUE}<br>⇒ y = "<br>Confidence: 0.998. Support: 289.` |
| 209 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {CALL}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 511.` |
| 210 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {CALL}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved = .<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {EXPRESSION}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 459.` |
| 211 | `  -1.internal_type = Identifier<br>	∧ -1.roles not in {CALL}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved not in {.}<br>	∧ +1.reserved not in {}}<br>	∧ ^2.internal_type = VariableDeclaration<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 640.` |
| 212 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.roles not in {CALL}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved not in {.}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^2.internal_type = VariableDeclaration<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 574.` |
| 213 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {CALL}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved not in {.}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 595.` |
| 214 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {CALL}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved not in {.}<br>	∧ -3.roles in {BINARY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 327.` |
| 215 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {CALL}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved not in {.}<br>	∧ -3.roles not in {BINARY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +2.roles in {ASSIGNMENT}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 351.` |
| 216 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {CALL}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved not in {.}<br>	∧ -3.roles not in {BINARY}<br>	∧ -4.reserved not in {'}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 147.` |
| 217 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {CALL}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved not in {.}<br>	∧ -3.roles in {EXPRESSION} and not in {BINARY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 893.` |
| 218 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {CALL}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved not in {.}<br>	∧ -3.roles not in {BINARY, EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 358.` |
| 219 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {BODY} and not in {DECLARATION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 239.` |
| 220 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {ARGUMENT, IDENTIFIER}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.roles not in {ARGUMENT}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {BODY} and not in {DECLARATION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 986.` |
| 221 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {QUALIFIED} and not in {BODY, DECLARATION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 7800.` |
| 222 | `  -1.diff_col ≥ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = .<br>	∧ ^1.roles not in {ARITHMETIC, BODY, DECLARATION, QUALIFIED}<br>	∧ ^2.internal_type not in {LogicalExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.984. Support: 221.` |
| 223 | `  -1.diff_col ≤ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = .<br>	∧ ^1.roles in {LIST} and not in {ARITHMETIC, BODY, DECLARATION, QUALIFIED}<br>	∧ ^2.internal_type not in {LogicalExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 404.` |
| 224 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {.}<br>	∧ +3.reserved = ,<br>	∧ ^1.roles not in {ARITHMETIC, BODY, DECLARATION, QUALIFIED}<br>	∧ ^2.internal_type not in {LogicalExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 2430.` |
| 225 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {LITERAL}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {.}<br>	∧ +3.reserved not in {,}<br>	∧ ^1.roles not in {ARITHMETIC, BODY, DECLARATION, QUALIFIED}<br>	∧ ^2.internal_type not in {LogicalExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 2713.` |
| 226 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 12<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {.}<br>	∧ +2.length ≥ 5<br>	∧ +3.reserved not in {,}<br>	∧ ^1.roles in {EXPRESSION} and not in {ARITHMETIC, BODY, DECLARATION, QUALIFIED}<br>	∧ ^2.internal_type not in {LogicalExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 432.` |
| 227 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {.}<br>	∧ +2.length ≤ 4<br>	∧ +3.reserved not in {,}<br>	∧ ^1.roles in {INSTANCE} and not in {ARITHMETIC, BODY, DECLARATION, QUALIFIED}<br>	∧ ^2.internal_type not in {LogicalExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 423.` |
| 228 | `  -3.diff_offset ≥ 4<br>	∧ +1.internal_type = CommentLine<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {.}<br>	∧ +2.length ≤ 4<br>	∧ +3.reserved not in {,}<br>	∧ ^1.roles not in {ARITHMETIC, BODY, DECLARATION, INSTANCE, MAP, QUALIFIED}<br>	∧ ^2.internal_type not in {LogicalExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 484.` |
| 229 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {.}<br>	∧ +3.reserved not in {,}<br>	∧ +5.reserved = new<br>	∧ ^1.roles not in {ARITHMETIC, BODY, DECLARATION, QUALIFIED}<br>	∧ ^2.internal_type = ArrayExpression<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 397.` |
| 230 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {.}<br>	∧ +3.reserved not in {,}<br>	∧ +5.reserved not in {new}<br>	∧ ^1.roles not in {ARITHMETIC, BODY, DECLARATION, QUALIFIED}<br>	∧ ^2.internal_type = ArrayExpression<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 857.` |
| 231 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 1048.` |
| 232 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +3.reserved = :<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 894.` |
| 233 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +3.reserved not in {:}<br>	∧ ^1.roles in {QUALIFIED} and not in {BLOCK, MAP}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 499.` |
| 234 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = -<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +3.reserved not in {:}<br>	∧ ^1.roles not in {BLOCK, MAP, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 409.` |
| 235 | `  -5.roles in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>	∧ +4.reserved = :<br>	∧ ^1.roles not in {BINARY}<br>⇒ y = "<br>Confidence: 0.996. Support: 875.` |
| 236 | `  -5.roles in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>	∧ +3.reserved = [<br>	∧ +4.reserved not in {:}<br>	∧ ^1.roles not in {BINARY}<br>⇒ y = "<br>Confidence: 0.991. Support: 957.` |
| 237 | `  -5.roles in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>	∧ +3.reserved not in {[}<br>	∧ +4.reserved not in {:}<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles in {LIST}<br>⇒ y = "<br>Confidence: 0.982. Support: 638.` |
| 238 | `  -2.roles in {STRING}<br>	∧ -3.diff_col ≥ 9<br>	∧ -5.roles not in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>⇒ y = "<br>Confidence: 0.993. Support: 1782.` |
| 239 | `  -1.diff_col ≥ 5<br>	∧ -1.roles in {KEY}<br>	∧ -4.diff_col ≥ 17<br>	∧ +1.roles not in {STRING}<br>⇒ y = "<br>Confidence: 0.981. Support: 2725.` |
| 240 | `  -1.roles in {VALUE} and not in {KEY}<br>	∧ -2.diff_offset ≥ 17<br>	∧ +1.roles not in {STRING}<br>	∧ +4.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.996. Support: 1695.` |
| 241 | `  -1.internal_type = CommentLine<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 17<br>	∧ +1.roles not in {STRING}<br>	∧ +4.roles not in {LITERAL}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ⏎<br>Confidence: 0.933. Support: 468.` |
| 242 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 17<br>	∧ -2.internal_type = CommentBlock<br>	∧ +1.roles not in {STRING}<br>	∧ +4.roles not in {LITERAL}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 141.` |
| 243 | `  -1.roles not in {KEY}<br>	∧ -1.length ≥ 3<br>	∧ -2.diff_offset ≤ 16<br>	∧ -3.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ +5.reserved not in {.}<br>	∧ ^1.roles not in {SCOPE, VARIABLE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 241.` |
| 244 | `  •••start_col ≤ 17<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -3.internal_type not in {Identifier}<br>	∧ -3.reserved not in {,}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.internal_type not in {Identifier}<br>	∧ ^1.roles in {OPERATOR} and not in {SCOPE, VARIABLE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 145.` |
| 245 | `  -1.reserved = (<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -3.internal_type not in {Identifier}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.internal_type not in {Identifier}<br>	∧ ^1.roles not in {OPERATOR, SCOPE, VARIABLE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 576.` |
| 246 | `  -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -4.diff_offset ≥ 56<br>	∧ +1.roles not in {STRING}<br>	∧ +4.reserved = [<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 830.` |
| 247 | `  -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ +1.roles not in {STRING}<br>	∧ +4.reserved not in {[}<br>	∧ ^1.roles in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 7727.` |
| 248 | `  •••start_col ≥ 13<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ +4.reserved not in {[}<br>	∧ +5.reserved = new<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 424.` |
| 249 | `  •••start_col ≥ 13<br>	∧ -1.roles not in {CALL, KEY}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ -4.diff_offset ≥ 11<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {IDENTIFIER, STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +4.reserved not in {[}<br>	∧ +5.reserved not in {new}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 1814.` |
| 250 | `  •••start_col ≥ 13<br>	∧ -1.roles not in {CALL, KEY}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ -4.diff_offset ≥ 11<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {IDENTIFIER, STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +4.reserved not in {[}<br>	∧ +5.reserved not in {new}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^2.roles in {INITIALIZATION} and not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 1496.` |
| 251 | `  •••start_col ≥ 13<br>	∧ -1.roles not in {CALL, KEY}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ -4.diff_offset ≥ 11<br>	∧ +1.reserved = new<br>	∧ +1.roles not in {IDENTIFIER, STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +4.reserved not in {[}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>	∧ ^2.roles not in {INITIALIZATION, STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.994. Support: 419.` |
| 252 | `  •••start_col ≥ 13<br>	∧ -1.roles not in {CALL, KEY}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ -4.diff_offset ≥ 11<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {IDENTIFIER, STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +4.reserved not in {[}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {INITIALIZATION, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 295.` |
| 253 | `  •••start_col ≥ 13<br>	∧ -1.roles not in {CALL, KEY}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ -4.diff_offset ≥ 11<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {IDENTIFIER, STRING}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ +4.reserved not in {[}<br>	∧ +5.reserved not in {new}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 7470.` |
| 254 | `  •••start_col ≥ 13<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ -4.diff_offset ≤ 10<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {RIGHT} and not in {STRING}<br>	∧ +4.reserved not in {[}<br>	∧ +4.roles not in {LITERAL}<br>	∧ ^1.roles in {EXPRESSION} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 183.` |
| 255 | `  •••start_col ≤ 12<br>	∧ -1.reserved = new<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ +4.reserved not in {[}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 401.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.866666666666667, "max_conf": 0.9998860359191895, "max_support": 11250, "min_conf": 0.921444833278656, "min_support": 134, "num_rules": 255}}
```
</details>
