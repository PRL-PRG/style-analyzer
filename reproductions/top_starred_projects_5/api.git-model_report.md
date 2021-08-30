# Model report for file:///tmp/top-repos-quality-repos-zqejew8w/api.git HEAD 3ba2eac0e7c9093852bdd3400c7f2581cd74ac9e

### Dump

```json
{'created_at': '2021-08-29 22:58:18',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '24.7 kB',
 'tags': [],
 'uuid': '1e7b70b0-5141-4fa2-98e2-7c4e34a1b0f6',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-zqejew8w/api.git 3ba2eac0e7c9093852bdd3400c7f2581cd74ac9e

# javascript
386 rules, avg.len. 9.0
## train
PPCR: 0.999643
### report
macro
{'f1-score': 0.8516337487808545,
 'precision': 0.9200251120724489,
 'recall': 0.8065656492120163,
 'support': 69937}
micro
{'f1-score': 0.9529004675636644,
 'precision': 0.9529004675636644,
 'recall': 0.9529004675636644,
 'support': 69937}
weighted
{'f1-score': 0.951295942934129,
 'precision': 0.9518903554606836,
 'recall': 0.9529004675636644,
 'support': 69937}
### report_full
macro
{'f1-score': 0.8513497990236996,
 'precision': 0.9200251120724489,
 'recall': 0.8061668808592197,
 'support': 69962}
micro
{'f1-score': 0.9527301839183983,
 'precision': 0.9529004675636644,
 'recall': 0.9525599611217518,
 'support': 69962}
weighted
{'f1-score': 0.951113043709267,
 'precision': 0.9518856294548195,
 'recall': 0.9525599611217518,
 'support': 69962}
## test
PPCR: 0.999336
### report
macro
{'f1-score': 0.7612042929284342,
 'precision': 0.8521378990738996,
 'recall': 0.719693978552649,
 'support': 15055}
micro
{'f1-score': 0.9337761541016273,
 'precision': 0.9337761541016274,
 'recall': 0.9337761541016274,
 'support': 15055}
weighted
{'f1-score': 0.9299357592808378,
 'precision': 0.9311346630536245,
 'recall': 0.9337761541016274,
 'support': 15055}
### report_full
macro
{'f1-score': 0.7607925964921627,
 'precision': 0.8521378990738996,
 'recall': 0.7191810267109453,
 'support': 15065}
micro
{'f1-score': 0.9334661354581674,
 'precision': 0.9337761541016274,
 'recall': 0.9331563226020577,
 'support': 15065}
weighted
{'f1-score': 0.9295964980334157,
 'precision': 0.9311133816866546,
 'recall': 0.9331563226020577,
 'support': 15065}
```

## javascript
### Summary
237 rules, avg.len. 8.9

| | |
|-|-|
|Min support|140|
|Max support|16661|
|Min confidence|0.9215440154075623|
|Max confidence|0.9999334812164307|

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
| 1 | `  -1.roles in {EXPRESSION}<br>	∧ -3.roles in {CALL}<br>	∧ -4.diff_offset ≥ 29<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.962. Support: 276.` |
| 2 | `  •••start_col ≤ 50<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 28<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = ArrayExpression<br>⇒ y = '<br>Confidence: 0.992. Support: 552.` |
| 3 | `  -1.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 8<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = ArrayExpression<br>⇒ y = ∅<br>Confidence: 0.999. Support: 376.` |
| 4 | `  -1.diff_offset ≤ 3<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.diff_col ≥ 14<br>	∧ -4.diff_offset ≤ 28<br>	∧ +3.internal_type = Identifier<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 220.` |
| 5 | `  -1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ -4.diff_offset ≤ 28<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.roles not in {CONDITION, DECLARATION}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 13430.` |
| 6 | `  -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = Identifier<br>	∧ +4.roles not in {CALL}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 6739.` |
| 7 | `  -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ +2.internal_type not in {Identifier}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1034.` |
| 8 | `  -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ +1.roles in {LITERAL} and not in {ARGUMENT}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 1257.` |
| 9 | `  -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.roles in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.988. Support: 705.` |
| 10 | `  -1.label in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.roles not in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 396.` |
| 11 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.roles in {CALLEE}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.961. Support: 216.` |
| 12 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.label not in {<newline>}<br>	∧ -4.roles not in {CALLEE}<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.948. Support: 1597.` |
| 13 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {NUMBER}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 4<br>	∧ +2.reserved = :<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 261.` |
| 14 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {NUMBER}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 4<br>	∧ +2.reserved = :<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>	∧ ^2.internal_type = AssignmentExpression<br>⇒ y = ␣<br>Confidence: 0.987. Support: 197.` |
| 15 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {NUMBER}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 4<br>	∧ +2.reserved = :<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>	∧ ^2.internal_type not in {AssignmentExpression}<br>	∧ ^2.roles in {RIGHT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 434.` |
| 16 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {,}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 3<br>	∧ +5.roles in {KEY}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 401.` |
| 17 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {,}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 3<br>	∧ +5.roles not in {KEY}<br>	∧ ^1.roles in {SCOPE} and not in {DECLARATION, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 3232.` |
| 18 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {,}<br>	∧ -4.roles in {LITERAL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 3<br>	∧ +4.roles in {MAP}<br>	∧ +5.roles not in {KEY}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED, SCOPE}<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 151.` |
| 19 | `  •••start_col ≤ 25<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {,}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 3<br>	∧ +4.roles not in {MAP}<br>	∧ +5.roles not in {KEY}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED, SCOPE}<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = ∅<br>Confidence: 0.955. Support: 1113.` |
| 20 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.959. Support: 3031.` |
| 21 | `  -1.roles not in {STRING}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 15173.` |
| 22 | `  -1.roles not in {STRING}<br>	∧ -3.label in {<space>}<br>	∧ -5.roles in {ARGUMENT, EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 1489.` |
| 23 | `  -1.roles not in {STRING}<br>	∧ -3.label in {<space>}<br>	∧ -5.roles in {EXPRESSION} and not in {ARGUMENT}<br>	∧ +1.roles in {MAP}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 765.` |
| 24 | `  -1.roles not in {STRING}<br>	∧ -3.label in {<space>}<br>	∧ -5.roles not in {EXPRESSION}<br>	∧ +1.roles in {VALUE}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 887.` |
| 25 | `  -1.roles not in {STRING}<br>	∧ -3.label in {<space>}<br>	∧ -5.roles not in {EXPRESSION}<br>	∧ +1.roles not in {VALUE}<br>	∧ +3.reserved = ><br>	∧ +4.reserved not in {const}<br>	∧ +4.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INITIALIZATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 206.` |
| 26 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.label not in {<space>}<br>	∧ ^1.roles in {BINARY} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 383.` |
| 27 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type = Identifier<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_col ≥ 7<br>	∧ -3.label not in {<space>}<br>	∧ +2.roles in {STRING}<br>	∧ +4.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {BINARY, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1821.` |
| 28 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type = Identifier<br>	∧ -1.roles not in {STRING, VALUE}<br>	∧ -3.diff_col ≥ 7<br>	∧ -3.label not in {<space>}<br>	∧ +2.roles not in {STRING}<br>	∧ +4.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {BINARY, IDENTIFIER}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.998. Support: 311.` |
| 29 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type = Identifier<br>	∧ -1.roles not in {STRING, VALUE}<br>	∧ -3.diff_col ≥ 7<br>	∧ -3.label not in {<space>}<br>	∧ +2.roles not in {STRING}<br>	∧ +4.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {BINARY, IDENTIFIER}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 3535.` |
| 30 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type = Identifier<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_col ≤ 6<br>	∧ -3.label not in {<space>}<br>	∧ +4.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {BINARY, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 2262.` |
| 31 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved = :<br>	∧ ^1.roles not in {BINARY, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 439.` |
| 32 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {:}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {BINARY, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 455.` |
| 33 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {:}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {BINARY, IDENTIFIER}<br>	∧ ^2.roles in {STATEMENT} and not in {BODY}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 171.` |
| 34 | `  -1.diff_col ≤ 1<br>	∧ -1.label in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.label not in {<space>}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 997.` |
| 35 | `  -1.diff_col ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved = )<br>	∧ -3.label not in {<space>}<br>	∧ +4.reserved = .<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 957.` |
| 36 | `  -1.diff_col ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved = )<br>	∧ -3.label not in {<space>}<br>	∧ -4.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +3.roles in {ARGUMENT}<br>	∧ +4.reserved not in {., }}<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.956. Support: 170.` |
| 37 | `  -1.diff_col ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<space>}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 436.` |
| 38 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<space>}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 545.` |
| 39 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 1.000. Support: 1340.` |
| 40 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 2494.` |
| 41 | `  •••start_col ≤ 5<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<space>}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.974. Support: 211.` |
| 42 | `  -1.diff_col ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<space>}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type = AssignmentExpression<br>⇒ y = ␣<br>Confidence: 0.952. Support: 199.` |
| 43 | `  -1.diff_col ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved = ><br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {AssignmentExpression}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.992. Support: 551.` |
| 44 | `  -1.diff_col ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<space>}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {ARGUMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles in {FUNCTION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 339.` |
| 45 | `  -1.diff_col ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {FUNCTION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 1140.` |
| 46 | `  -1.diff_col ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<space>}<br>	∧ -5.reserved = const<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {FUNCTION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 193.` |
| 47 | `  -1.diff_col ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<space>}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles in {BODY} and not in {FUNCTION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 456.` |
| 48 | `  -1.diff_col ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles in {LITERAL}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {BODY, FUNCTION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 182.` |
| 49 | `  -1.internal_type = Identifier<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ ^1.roles in {BINARY}<br>	∧ ^2.roles in {BINARY}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 393.` |
| 50 | `  •••start_line ≥ 7<br>	∧ -1.diff_col ≥ 3<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {BINARY}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 176.` |
| 51 | `  •••start_line ≥ 7<br>	∧ -1.diff_col ≥ 3<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {BINARY}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 8381.` |
| 52 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ +2.reserved = get<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {BINARY}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.978. Support: 248.` |
| 53 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ +1.length ≤ 2<br>	∧ +2.reserved not in {get}<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ +4.reserved = (<br>	∧ +5.reserved not in {>}<br>	∧ ^1.roles not in {BINARY}<br>⇒ y = ⏎<br>Confidence: 0.945. Support: 229.` |
| 54 | `  -1.reserved = :<br>	∧ +2.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 1677.` |
| 55 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ +2.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 1.000. Support: 1053.` |
| 56 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles in {STRING}<br>	∧ +2.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.972. Support: 2393.` |
| 57 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {STRING}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^2.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 3659.` |
| 58 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^2.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 201.` |
| 59 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ +4.reserved = ,<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.951. Support: 193.` |
| 60 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {MAP}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {INITIALIZATION} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 1076.` |
| 61 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles in {IDENTIFIER} and not in {STRING}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type = VariableDeclaration<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 295.` |
| 62 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles in {IDENTIFIER} and not in {STRING}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 6521.` |
| 63 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 4022.` |
| 64 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.internal_type = Identifier<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {CALL}<br>	∧ ^2.roles in {CALL} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 327.` |
| 65 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -4.reserved = {<br>	∧ +1.internal_type = Identifier<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {CALL}<br>	∧ ^2.roles not in {CALL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 214.` |
| 66 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CALL}<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 212.` |
| 67 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.length ≤ 6<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 683.` |
| 68 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.length ≤ 6<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {>}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 1570.` |
| 69 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.length ≤ 6<br>	∧ -5.internal_type = Identifier<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.roles in {STRING}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ +4.internal_type = Identifier<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 177.` |
| 70 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.length ≤ 6<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ +4.internal_type = Identifier<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^2.roles in {EXPRESSION} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 743.` |
| 71 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.length ≤ 6<br>	∧ -5.roles not in {CALL}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ +4.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 536.` |
| 72 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.length ≤ 6<br>	∧ -5.roles not in {CALL}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved not in {), >}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles in {CALL} and not in {IDENTIFIER}<br>	∧ +4.internal_type not in {Identifier}<br>	∧ +5.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {BlockStatement, Program}<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 220.` |
| 73 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.length ≤ 6<br>	∧ -4.roles in {KEY}<br>	∧ -5.roles not in {CALL}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved not in {), >}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {CALL, IDENTIFIER}<br>	∧ +4.internal_type not in {Identifier}<br>	∧ +5.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {BlockStatement, Program}<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 146.` |
| 74 | `  -2.reserved = )<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.roles in {BLOCK, FUNCTION}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 1023.` |
| 75 | `  -2.reserved = )<br>	∧ -5.label in {<newline>}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {BLOCK, FUNCTION}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.982. Support: 141.` |
| 76 | `  -2.reserved = )<br>	∧ ^1.roles in {BLOCK} and not in {FUNCTION}<br>⇒ y = ⏎<br>Confidence: 0.945. Support: 411.` |
| 77 | `  -2.reserved not in {)}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {{}<br>	∧ +2.reserved not in {get}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.994. Support: 9332.` |
| 78 | `  -1.length ≥ 2<br>	∧ -2.reserved not in {)}<br>	∧ -2.length ≤ 1<br>	∧ -5.label not in {<+tab>}<br>	∧ +1.reserved not in {{}<br>	∧ +2.reserved not in {get}<br>	∧ +5.roles in {IDENTIFIER}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.957. Support: 733.` |
| 79 | `  -1.length ≤ 1<br>	∧ -2.reserved not in {)}<br>	∧ -2.length ≤ 1<br>	∧ -3.internal_type = StringLiteral<br>	∧ -5.label not in {<+tab>}<br>	∧ +1.reserved not in {{}<br>	∧ +2.reserved not in {get}<br>	∧ +5.roles in {IDENTIFIER}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ⏎<br>Confidence: 0.929. Support: 231.` |
| 80 | `  -1.length ≤ 1<br>	∧ -2.reserved not in {)}<br>	∧ -2.length ≤ 1<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -5.label not in {<+tab>}<br>	∧ +1.reserved not in {{}<br>	∧ +2.reserved not in {get}<br>	∧ +5.roles in {IDENTIFIER}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.950. Support: 248.` |
| 81 | `  -2.reserved not in {)}<br>	∧ -2.length ≤ 1<br>	∧ -5.label not in {<+tab>}<br>	∧ +1.reserved not in {{}<br>	∧ +2.reserved not in {get}<br>	∧ +5.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.970. Support: 4901.` |
| 82 | `  -2.reserved = .<br>	∧ +1.reserved not in {{}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 3838.` |
| 83 | `  -2.reserved not in {), .}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 3192.` |
| 84 | `  -1.reserved = :<br>	∧ -2.reserved not in {), .}<br>	∧ +1.reserved not in {;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 1375.` |
| 85 | `  -1.reserved = (<br>	∧ -2.reserved not in {), .}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ +4.internal_type = Identifier<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 429.` |
| 86 | `  -1.reserved = (<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.reserved not in {;, {}<br>	∧ +1.length ≥ 16<br>	∧ +4.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = '<br>Confidence: 0.928. Support: 145.` |
| 87 | `  -1.reserved = (<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≤ 4<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ +4.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 500.` |
| 88 | `  -1.reserved not in {(, :}<br>	∧ -2.reserved = :<br>	∧ -3.diff_col ≥ 4<br>	∧ -3.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 676.` |
| 89 | `  -1.reserved = const<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≥ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 162.` |
| 90 | `  -1.reserved not in {(, :}<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≥ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {;}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 1861.` |
| 91 | `  -1.reserved not in {(, :}<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≥ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {;}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 679.` |
| 92 | `  -2.reserved not in {), .}<br>	∧ +1.reserved not in {;, {}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 775.` |
| 93 | `  -1.roles in {ARGUMENT, LITERAL}<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≥ 8<br>	∧ -5.reserved not in {{}<br>	∧ +1.reserved not in {;, {}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = '<br>Confidence: 0.943. Support: 1413.` |
| 94 | `  -1.roles in {ARGUMENT} and not in {LITERAL}<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≥ 8<br>	∧ -5.reserved not in {{}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 436.` |
| 95 | `  -1.roles not in {ARGUMENT}<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≥ 8<br>	∧ -5.reserved = ;<br>	∧ +1.reserved not in {;, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 295.` |
| 96 | `  -1.roles in {STRING} and not in {ARGUMENT}<br>	∧ -1.length ≥ 5<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≥ 8<br>	∧ -4.diff_offset ≥ 15<br>	∧ -5.reserved not in {;}<br>	∧ +1.reserved not in {;, {}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BLOCK}<br>⇒ y = '<br>Confidence: 0.935. Support: 409.` |
| 97 | `  -1.reserved = :<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≤ 7<br>	∧ +1.reserved not in {;, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 166.` |
| 98 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {:}<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≤ 7<br>	∧ +1.reserved not in {;, {}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.999. Support: 469.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≤ 7<br>	∧ -5.diff_offset ≥ 9<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>	∧ ^2.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 169.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≤ 7<br>	∧ -5.diff_offset ≤ 8<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 514.` |
| 101 | `  -1.reserved not in {:}<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≤ 7<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +5.roles in {CALL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 730.` |
| 102 | `  -1.reserved not in {:}<br>	∧ -1.roles in {ARGUMENT}<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≤ 7<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +5.roles not in {CALL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 1242.` |
| 103 | `  -1.reserved not in {:}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.reserved not in {), .}<br>	∧ -3.diff_col ≤ 7<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +5.roles not in {CALL}<br>	∧ ^1.internal_type = ObjectProperty<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 1066.` |
| 104 | `  -1.roles in {STRING}<br>	∧ ^1.roles not in {VARIABLE}<br>	∧ ^2.roles in {LIST}<br>⇒ y = '<br>Confidence: 0.999. Support: 602.` |
| 105 | `  -1.roles in {STRING}<br>	∧ ^1.roles in {CALLEE} and not in {VARIABLE}<br>	∧ ^2.roles not in {LIST}<br>⇒ y = '<br>Confidence: 0.984. Support: 339.` |
| 106 | `  -1.roles in {STRING} and not in {VALUE}<br>	∧ ^1.roles not in {CALLEE, VARIABLE}<br>	∧ ^2.roles not in {LIST}<br>⇒ y = '<br>Confidence: 1.000. Support: 1922.` |
| 107 | `  -1.roles in {IDENTIFIER} and not in {STRING}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 15884.` |
| 108 | `  -1.roles not in {IDENTIFIER, STRING}<br>	∧ -3.reserved not in {(}<br>	∧ -3.length ≥ 10<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ ^1.roles in {CALLEE} and not in {VARIABLE}<br>⇒ y = ⏎<br>Confidence: 0.933. Support: 231.` |
| 109 | `  -1.roles not in {IDENTIFIER, STRING}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {CALLEE} and not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 3082.` |
| 110 | `  -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {CALLEE, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 1193.` |
| 111 | `  -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -4.roles in {NUMBER}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {CALLEE, VARIABLE}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 168.` |
| 112 | `  -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -4.roles not in {NUMBER}<br>	∧ -5.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {CALLEE, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 737.` |
| 113 | `  -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -4.roles not in {NUMBER}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {CALLEE, VARIABLE}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 670.` |
| 114 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -4.roles in {CALL}<br>	∧ +1.reserved = }<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {CALLEE, VARIABLE}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.982. Support: 252.` |
| 115 | `  -1.reserved not in {,}<br>	∧ -1.roles in {CALL} and not in {IDENTIFIER, STRING}<br>	∧ -4.roles in {CALL}<br>	∧ +1.reserved not in {}}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {CALLEE, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 197.` |
| 116 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.diff_col ≥ 3<br>	∧ -4.roles not in {CALL}<br>	∧ -5.diff_col ≤ 8<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {MAP} and not in {CALLEE}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 399.` |
| 117 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.diff_col ≤ 3<br>	∧ -4.roles not in {CALL}<br>	∧ -5.diff_col ≤ 8<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {MAP} and not in {CALLEE, VARIABLE}<br>⇒ y = '<br>Confidence: 0.999. Support: 464.` |
| 118 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -4.roles not in {CALL}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {CALLEE, MAP, VARIABLE}<br>⇒ y = '<br>Confidence: 0.968. Support: 2047.` |
| 119 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {CALLEE, VARIABLE}<br>	∧ ^2.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 4368.` |
| 120 | `  -1.reserved not in {,, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {CALLEE, VARIABLE}<br>	∧ ^2.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 1465.` |
| 121 | `  -1.reserved not in {,, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {CALLEE}<br>	∧ ^2.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 683.` |
| 122 | `  -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles not in {CALLEE}<br>	∧ ^2.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 994.` |
| 123 | `  -1.reserved not in {:, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {EXPRESSION} and not in {CALLEE}<br>	∧ ^2.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 224.` |
| 124 | `  -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {CALLEE, VARIABLE}<br>	∧ ^2.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 3002.` |
| 125 | `  -1.reserved not in {:, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -4.roles not in {CALL}<br>	∧ -5.roles in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.roles not in {CALLEE}<br>	∧ ^2.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 620.` |
| 126 | `  -1.reserved not in {:, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT} and not in {CALLEE}<br>	∧ ^2.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 359.` |
| 127 | `  -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.internal_type = Identifier<br>	∧ -2.reserved not in {)}<br>	∧ -4.roles not in {CALL}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, =, }}<br>	∧ ^1.roles not in {CALLEE, STATEMENT, VARIABLE}<br>	∧ ^2.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 2709.` |
| 128 | `  -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -4.roles not in {CALL}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {CALLEE, STATEMENT, VARIABLE}<br>	∧ ^2.roles in {EXPRESSION} and not in {FUNCTION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 626.` |
| 129 | `  -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {)}<br>	∧ -4.roles in {ARGUMENT} and not in {CALL}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ ^1.roles not in {CALLEE, STATEMENT, VARIABLE}<br>	∧ ^2.roles in {EXPRESSION} and not in {FUNCTION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 662.` |
| 130 | `  -1.roles in {EXPRESSION, IDENTIFIER}<br>	∧ -2.reserved = (<br>	∧ +3.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 429.` |
| 131 | `  -1.roles in {EXPRESSION} and not in {IDENTIFIER}<br>	∧ -2.reserved = (<br>	∧ -5.diff_offset ≥ 16<br>	∧ +3.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.984. Support: 1258.` |
| 132 | `  -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.roles in {VARIABLE} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 140.` |
| 133 | `  -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 917.` |
| 134 | `  -1.roles in {EXPRESSION}<br>	∧ -3.reserved = :<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ +4.internal_type = StringLiteral<br>	∧ +4.length ≥ 2<br>⇒ y = '<br>Confidence: 0.968. Support: 428.` |
| 135 | `  -1.roles in {EXPRESSION, STRING}<br>	∧ -3.reserved = :<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ +4.internal_type not in {StringLiteral}<br>	∧ +4.length ≥ 2<br>⇒ y = '<br>Confidence: 0.998. Support: 210.` |
| 136 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -3.reserved = :<br>	∧ +2.length ≥ 2<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ +4.internal_type not in {StringLiteral}<br>	∧ +4.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.983. Support: 323.` |
| 137 | `  -1.roles in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ -3.reserved not in {:}<br>	∧ -5.diff_offset ≤ 31<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 211.` |
| 138 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {:}<br>	∧ -5.diff_offset ≤ 31<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.998. Support: 258.` |
| 139 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {:}<br>	∧ -5.diff_offset ≤ 31<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 12529.` |
| 140 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 1.000. Support: 7518.` |
| 141 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +5.roles not in {FUNCTION}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^2.internal_type not in {CallExpression}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 430.` |
| 142 | `  -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = '<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 1260.` |
| 143 | `  -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = .<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.980. Support: 1276.` |
| 144 | `  -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -3.reserved not in {', .}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 664.` |
| 145 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {'}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>	∧ +4.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.968. Support: 173.` |
| 146 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label in {<space>}<br>	∧ -3.reserved not in {'}<br>	∧ -4.roles in {NUMBER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 170.` |
| 147 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label in {<space>}<br>	∧ -3.reserved not in {'}<br>	∧ -4.roles not in {NUMBER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 166.` |
| 148 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {', (}<br>	∧ -4.reserved = =<br>	∧ -5.internal_type not in {Identifier}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.949. Support: 620.` |
| 149 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {', (, {}<br>	∧ -4.reserved not in {=}<br>	∧ -5.internal_type not in {Identifier}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {:}<br>	∧ +3.length ≥ 18<br>	∧ +4.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 212.` |
| 150 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {', (, {}<br>	∧ -4.diff_col ≤ 10<br>	∧ -4.reserved not in {=}<br>	∧ -5.internal_type not in {Identifier}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {:}<br>	∧ +4.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BLOCK}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 213.` |
| 151 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>}<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {', (, {}<br>	∧ -4.reserved not in {=}<br>	∧ -5.internal_type not in {Identifier}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {:}<br>	∧ +4.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BLOCK}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 1830.` |
| 152 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {', (, {}<br>	∧ -4.diff_col ≥ 17<br>	∧ -4.reserved not in {=}<br>	∧ -5.internal_type not in {Identifier}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {:}<br>	∧ +3.reserved not in {)}<br>	∧ +4.reserved not in {(}<br>	∧ +5.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {CALL, FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 218.` |
| 153 | `  -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -3.diff_line = 0<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {', (, {}<br>	∧ -4.diff_col ≥ 17<br>	∧ -4.reserved not in {=}<br>	∧ -5.internal_type not in {Identifier}<br>	∧ +1.roles not in {EXPRESSION, STRING}<br>	∧ +2.reserved not in {:}<br>	∧ +3.reserved not in {)}<br>	∧ +4.reserved not in {(}<br>	∧ +5.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {CALL, FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 258.` |
| 154 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 1.000. Support: 2978.` |
| 155 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 13991.` |
| 156 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles in {MAP}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 1780.` |
| 157 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles in {IDENTIFIER} and not in {MAP}<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.964. Support: 1253.` |
| 158 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles not in {MAP}<br>	∧ -5.roles in {CALL}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 219.` |
| 159 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles not in {MAP}<br>	∧ -5.roles not in {CALL}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +5.roles in {LITERAL}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 268.` |
| 160 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles not in {MAP}<br>	∧ -5.roles not in {CALL}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +5.roles in {LITERAL}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 190.` |
| 161 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {KEY}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1382.` |
| 162 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.reserved = .<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2978.` |
| 163 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {CALL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 735.` |
| 164 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles not in {MAP}<br>	∧ -5.roles in {FUNCTION}<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +5.reserved = (<br>	∧ ^1.roles not in {BODY, IDENTIFIER}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 230.` |
| 165 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {BINARY}<br>	∧ +5.reserved not in {(}<br>	∧ ^1.roles not in {BODY, IDENTIFIER}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 178.` |
| 166 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -4.roles in {CALL} and not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved = )<br>	∧ +5.roles not in {KEY}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.959. Support: 231.` |
| 167 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -3.roles in {CALL}<br>	∧ -4.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {)}<br>	∧ +5.roles not in {KEY}<br>	∧ ^1.roles in {SCOPE}<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 221.` |
| 168 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -3.roles not in {CALL}<br>	∧ -4.label in {<newline>}<br>	∧ -4.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 2<br>	∧ +2.reserved not in {)}<br>	∧ +5.roles not in {KEY}<br>	∧ ^1.roles in {SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.968. Support: 142.` |
| 169 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -3.roles not in {CALL}<br>	∧ -4.label not in {<newline>}<br>	∧ -4.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {)}<br>	∧ +5.roles not in {KEY}<br>	∧ ^1.roles in {SCOPE}<br>⇒ y = ⏎<br>Confidence: 0.925. Support: 497.` |
| 170 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -4.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {)}<br>	∧ +5.roles not in {KEY}<br>	∧ ^1.roles in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 2313.` |
| 171 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -4.roles not in {STRING}<br>	∧ -5.diff_col ≥ 6<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved = {<br>	∧ +5.roles not in {KEY}<br>	∧ ^1.roles in {FUNCTION} and not in {SCOPE}<br>	∧ ^2.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 545.` |
| 172 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -4.roles not in {STRING}<br>	∧ -5.diff_col ≤ 5<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {)}<br>	∧ +5.roles not in {KEY}<br>	∧ ^1.roles in {FUNCTION} and not in {SCOPE}<br>	∧ ^2.roles not in {DECLARATION}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.929. Support: 539.` |
| 173 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -4.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {)}<br>	∧ +5.roles not in {FUNCTION, KEY}<br>	∧ ^1.roles in {QUALIFIED} and not in {FUNCTION, SCOPE}<br>	∧ ^2.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 1114.` |
| 174 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -4.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {ARGUMENT, EXPRESSION} and not in {STRING}<br>	∧ +2.reserved not in {)}<br>	∧ +5.roles not in {FUNCTION, KEY}<br>	∧ ^1.roles not in {FUNCTION, QUALIFIED, SCOPE}<br>	∧ ^2.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 247.` |
| 175 | `  -2.diff_col ≥ 3<br>	∧ -3.reserved not in {.}<br>	∧ +1.roles in {LITERAL}<br>	∧ +4.roles not in {EXPRESSION}<br>	∧ ^2.roles in {VARIABLE}<br>⇒ y = '<br>Confidence: 0.927. Support: 172.` |
| 176 | `  -2.diff_col ≥ 3<br>	∧ -3.reserved not in {.}<br>	∧ -5.label in {'}<br>	∧ +1.roles in {LITERAL}<br>	∧ +4.roles not in {EXPRESSION}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 570.` |
| 177 | `  -2.diff_col ≥ 3<br>	∧ -3.reserved not in {.}<br>	∧ -5.label not in {'}<br>	∧ +1.roles in {LITERAL}<br>	∧ +4.roles not in {EXPRESSION}<br>	∧ ^2.internal_type = ObjectExpression<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 609.` |
| 178 | `  -2.diff_col ≥ 3<br>	∧ -3.reserved not in {.}<br>	∧ -5.label not in {'}<br>	∧ -5.roles in {EXPRESSION}<br>	∧ +1.roles in {LITERAL}<br>	∧ +4.roles not in {EXPRESSION}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 259.` |
| 179 | `  +1.roles not in {LITERAL}<br>	∧ +2.reserved = ><br>⇒ y = ␣<br>Confidence: 0.973. Support: 753.` |
| 180 | `  -1.reserved not in {)}<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles in {CALLEE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 5764.` |
| 181 | `  -2.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CALLEE}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 776.` |
| 182 | `  -1.reserved = ,<br>	∧ -2.reserved not in {=}<br>	∧ -4.roles in {LITERAL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CALLEE}<br>⇒ y = ⏎<br>Confidence: 0.980. Support: 177.` |
| 183 | `  -1.reserved = ,<br>	∧ -2.internal_type = NumericLiteral<br>	∧ -2.reserved not in {=}<br>	∧ -4.roles not in {LITERAL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {LITERAL} and not in {CALLEE}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 424.` |
| 184 | `  -1.reserved = ,<br>	∧ -2.internal_type not in {NumericLiteral}<br>	∧ -4.roles not in {LITERAL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {LITERAL} and not in {CALLEE}<br>	∧ ^2.roles in {RIGHT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 571.` |
| 185 | `  -1.reserved = ,<br>	∧ -2.reserved not in {=}<br>	∧ -4.roles not in {LITERAL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {CALLEE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 1096.` |
| 186 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {,}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {VARIABLE} and not in {CALLEE}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 448.` |
| 187 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {,}<br>	∧ -2.reserved not in {=}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CALLEE, RELATIONAL, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 13006.` |
| 188 | `  •••start_col ≥ 11<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved = ;<br>	∧ -2.reserved not in {=}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = )<br>	∧ ^1.roles not in {CALLEE}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.994. Support: 270.` |
| 189 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = ;<br>	∧ -2.reserved not in {=}<br>	∧ -4.diff_col ≥ 3<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {), >}<br>	∧ ^1.roles in {SCOPE} and not in {CALLEE}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 1891.` |
| 190 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ -4.diff_offset ≥ 9<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CALLEE}<br>⇒ y = '<br>Confidence: 0.958. Support: 1478.` |
| 191 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ -4.diff_col ≤ 15<br>	∧ -4.diff_offset ≥ 9<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {>}<br>	∧ +5.reserved = ,<br>	∧ ^1.roles not in {CALLEE}<br>⇒ y = '<br>Confidence: 0.946. Support: 495.` |
| 192 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ -4.diff_offset ≤ 8<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {>}<br>	∧ +5.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {CALLEE}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 406.` |
| 193 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ -4.diff_offset ≤ 8<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {>}<br>	∧ +2.roles in {MAP}<br>	∧ +5.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {CALLEE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 374.` |
| 194 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ -4.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type = ArrowFunctionExpression<br>	∧ ^1.roles not in {CALLEE}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.968. Support: 574.` |
| 195 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ -4.reserved not in {=}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {CALLEE}<br>	∧ ^2.internal_type = ArrowFunctionExpression<br>⇒ y = ␣<br>Confidence: 0.973. Support: 237.` |
| 196 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ -4.reserved not in {=}<br>	∧ -4.length ≥ 2<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {CALLEE}<br>	∧ ^2.internal_type not in {ArrowFunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 243.` |
| 197 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ -4.diff_offset ≤ 12<br>	∧ -4.reserved not in {=}<br>	∧ -4.length ≤ 1<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {CALLEE}<br>	∧ ^2.internal_type not in {ArrowFunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 591.` |
| 198 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.diff_col ≤ 38<br>	∧ -2.reserved not in {=}<br>	∧ -4.reserved not in {=}<br>	∧ -5.roles not in {VALUE}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {CALLEE, IF}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 4522.` |
| 199 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.diff_col ≤ 38<br>	∧ -2.reserved not in {=}<br>	∧ -4.reserved not in {=}<br>	∧ -5.roles not in {VALUE}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {AssignmentExpression, MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {BOOLEAN, IF}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2609.` |
| 200 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^2.internal_type not in {AssignmentExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 654.` |
| 201 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {RELATIONAL}<br>	∧ ^2.internal_type not in {AssignmentExpression}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 16661.` |
| 202 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≥ 19<br>	∧ +2.length ≥ 3<br>	∧ +5.roles in {IDENTIFIER}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ⏎<br>Confidence: 0.991. Support: 158.` |
| 203 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.994. Support: 7507.` |
| 204 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.roles in {CALL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 375.` |
| 205 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label in {<space>}<br>	∧ -4.roles not in {CALL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 1927.` |
| 206 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label not in {<space>}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {MAP}<br>	∧ ^2.roles in {ASSIGNMENT}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 632.` |
| 207 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {FUNCTION}<br>	∧ -4.roles not in {CALL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 1015.` |
| 208 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = NumericLiteral<br>	∧ -2.roles not in {FUNCTION}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.roles in {LITERAL}<br>	∧ +3.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 193.` |
| 209 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {NumericLiteral}<br>	∧ -3.roles in {CALL}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.roles in {LITERAL}<br>	∧ +3.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.967. Support: 227.` |
| 210 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {NumericLiteral}<br>	∧ -3.roles not in {CALL}<br>	∧ -4.roles not in {CALL}<br>	∧ -5.diff_offset ≥ 15<br>	∧ +1.roles in {LITERAL}<br>	∧ +3.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.955. Support: 783.` |
| 211 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ -4.roles not in {CALL}<br>	∧ -5.roles in {CALL}<br>	∧ +1.roles in {LITERAL}<br>	∧ +3.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 266.` |
| 212 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +3.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {MAP}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 1838.` |
| 213 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.label in {<space>}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ +3.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 630.` |
| 214 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ +3.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 191.` |
| 215 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = Identifier<br>	∧ -2.roles not in {FUNCTION}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ +3.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {EXPRESSION} and not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 1426.` |
| 216 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ -4.diff_offset ≤ 8<br>	∧ -4.reserved = )<br>	∧ -4.roles not in {CALL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ +3.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {EXPRESSION} and not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 141.` |
| 217 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ -4.diff_offset ≤ 8<br>	∧ -4.reserved not in {)}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.roles in {ARGUMENT} and not in {LITERAL}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ +3.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {EXPRESSION} and not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 480.` |
| 218 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ -4.diff_offset ≤ 8<br>	∧ -4.reserved not in {)}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.roles not in {ARGUMENT, LITERAL}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ +3.reserved not in {;}<br>	∧ +4.reserved = :<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {EXPRESSION} and not in {CALL, MAP}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 171.` |
| 219 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ -4.roles not in {CALL}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ +3.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {EXPRESSION, MAP, VARIABLE}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.962. Support: 250.` |
| 220 | `  -1.diff_col ≤ 19<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved = .<br>	∧ -4.diff_col ≥ 11<br>	∧ +2.reserved = ;<br>⇒ y = ∅<br>Confidence: 0.998. Support: 257.` |
| 221 | `  -1.diff_col ≤ 19<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.diff_offset ≤ 9<br>	∧ +2.reserved not in {;}<br>	∧ +5.reserved = ,<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 191.` |
| 222 | `  -1.diff_col ≤ 19<br>	∧ -1.internal_type = Identifier<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {;}<br>	∧ +5.reserved not in {,}<br>	∧ ^1.roles not in {RELATIONAL}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 13786.` |
| 223 | `  -1.roles not in {EXPRESSION}<br>	∧ -3.label in {<space>}<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 1745.` |
| 224 | `  -1.roles not in {EXPRESSION}<br>	∧ -3.label not in {<space>}<br>	∧ -4.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.999. Support: 636.` |
| 225 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^2.roles in {ARGUMENT} and not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.979. Support: 1007.` |
| 226 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≤ 1<br>	∧ +2.reserved = (<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.991. Support: 171.` |
| 227 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +5.roles not in {FUNCTION}<br>	∧ ^1.roles in {QUALIFIED}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 8040.` |
| 228 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = (<br>	∧ +3.reserved = {<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 287.` |
| 229 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +3.reserved not in {{}<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 575.` |
| 230 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.roles not in {MAP}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {ARGUMENT} and not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 196.` |
| 231 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 1<br>	∧ +1.roles not in {MAP}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {ARGUMENT} and not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 2785.` |
| 232 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {MAP}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {ARGUMENT, LITERAL}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.986. Support: 837.` |
| 233 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -5.diff_col ≤ 15<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 5<br>	∧ +3.internal_type not in {Identifier}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {ARGUMENT, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 233.` |
| 234 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -2.length ≥ 2<br>	∧ -5.diff_col ≤ 15<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 6<br>	∧ +3.internal_type not in {Identifier}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {ARGUMENT, LITERAL}<br>⇒ y = '<br>Confidence: 1.000. Support: 1269.` |
| 235 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.length ≥ 3<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 4<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {ARGUMENT, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 1263.` |
| 236 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label in {<space>}<br>	∧ -5.length ≤ 5<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 4<br>	∧ +3.reserved not in {{}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {ARGUMENT, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 809.` |
| 237 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label not in {<space>}<br>	∧ -5.length ≤ 5<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 4<br>	∧ +2.reserved = ,<br>	∧ +3.reserved not in {{}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {ARGUMENT, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 178.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.852320675105485, "max_conf": 0.9999334812164307, "max_support": 16661, "min_conf": 0.9215440154075623, "min_support": 140, "num_rules": 237}}
```
</details>
