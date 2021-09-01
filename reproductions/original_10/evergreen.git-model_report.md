# Model report for file:///tmp/top-repos-quality-repos-ce0ke025/evergreen.git HEAD 27ab984aeba4b89dd6921dab17804c0fedae5f47

### Dump

```json
{'created_at': '2021-08-17 17:05:08',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-80-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.9 (default, Jan 26 2021, 15:33:00) [GCC 8.4.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '25.2 kB',
 'tags': [],
 'uuid': '9cbd3795-88d2-49f4-bb83-a584e6ec3111',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ce0ke025/evergreen.git 27ab984aeba4b89dd6921dab17804c0fedae5f47

# javascript
270 rules, avg.len. 7.9
## train
PPCR: 0.968224
### report
macro
{'f1-score': 0.617226301724473,
 'precision': 0.6324662379102937,
 'recall': 0.6093914293300959,
 'support': 139647}
micro
{'f1-score': 0.9251541386495951,
 'precision': 0.9251541386495951,
 'recall': 0.9251541386495951,
 'support': 139647}
weighted
{'f1-score': 0.9208509316298806,
 'precision': 0.9191145101720166,
 'recall': 0.9251541386495951,
 'support': 139647}
### report_full
macro
{'f1-score': 0.5930826825956949,
 'precision': 0.6324662379102937,
 'recall': 0.5690845776006965,
 'support': 144230}
micro
{'f1-score': 0.9102181578641454,
 'precision': 0.9251541386495951,
 'recall': 0.8957567773694793,
 'support': 144230}
weighted
{'f1-score': 0.9034581741136304,
 'precision': 0.9160623064071496,
 'recall': 0.8957567773694793,
 'support': 144230}
## test
PPCR: 0.967788
### report
macro
{'f1-score': 0.5280074603547434,
 'precision': 0.541004396540978,
 'recall': 0.5179068300837293,
 'support': 33830}
micro
{'f1-score': 0.9026899201891811,
 'precision': 0.9026899201891811,
 'recall': 0.9026899201891811,
 'support': 33830}
weighted
{'f1-score': 0.8944135797454369,
 'precision': 0.8877088068851241,
 'recall': 0.9026899201891811,
 'support': 33830}
### report_full
macro
{'f1-score': 0.5033052743848553,
 'precision': 0.541004396540978,
 'recall': 0.4778426995439355,
 'support': 34956}
micro
{'f1-score': 0.8879132381589276,
 'precision': 0.9026899201891811,
 'recall': 0.8736125414807187,
 'support': 34956}
weighted
{'f1-score': 0.8745440860954983,
 'precision': 0.8797780949455611,
 'recall': 0.8736125414807187,
 'support': 34956}
```

## javascript
### Summary
270 rules, avg.len. 7.9

| | |
|-|-|
|Min support|131|
|Max support|26980|
|Min confidence|0.8008061051368713|
|Max confidence|0.9994369149208069|

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
               'min_samples_leaf_max': 130,
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
                     'min_samples_leaf': 90,
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.984. Support: 22485.` |
| 2 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.813. Support: 1411.` |
| 3 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 4605.` |
| 4 | `  -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.877. Support: 3222.` |
| 5 | `  -1.reserved = {<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_col ≤ 3<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.858. Support: 341.` |
| 6 | `  -1.reserved not in {(, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.943. Support: 1990.` |
| 7 | `  -1.reserved not in {(, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.926. Support: 2884.` |
| 8 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.internal_type = StringLiteral<br>	∧ -4.length ≥ 6<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.963. Support: 714.` |
| 9 | `  •••start_line ≥ 219<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.837. Support: 843.` |
| 10 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 899.` |
| 11 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 714.` |
| 12 | `  •••start_col ≤ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BLOCK} and not in {FILE}<br>⇒ y = ⏎⏎<br>Confidence: 0.817. Support: 205.` |
| 13 | `  -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = '<br>Confidence: 0.932. Support: 287.` |
| 14 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 11432.` |
| 15 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = "<br>Confidence: 0.953. Support: 1957.` |
| 16 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {MAP}<br>	∧ -4.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = "<br>Confidence: 0.948. Support: 702.` |
| 17 | `  -1.internal_type = StringLiteral<br>	∧ -4.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {IF}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>	∧ ^2.roles not in {ASSIGNMENT}<br>⇒ y = '<br>Confidence: 0.808. Support: 2972.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -5.reserved = {<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.911. Support: 186.` |
| 19 | `  •••start_col ≤ 41<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION} and not in {NUMBER}<br>	∧ -5.reserved not in {;, {}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.875. Support: 292.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION, NUMBER}<br>	∧ -5.reserved not in {;, {}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.872. Support: 3336.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 3900.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LIST}<br>⇒ y = ⏎<br>Confidence: 0.818. Support: 449.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LIST}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 306.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LIST}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 3615.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 226.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -3.label not in {<space>}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = ␣<br>Confidence: 0.958. Support: 180.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.label not in {<space>}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 1384.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 1188.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.867. Support: 680.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 639.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.841. Support: 596.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, if}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.896. Support: 255.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -2.length ≥ 10<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.932. Support: 155.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, function, if}<br>	∧ -2.length ≤ 10<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.913. Support: 502.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 26146.` |
| 36 | `  -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 22830.` |
| 37 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 3051.` |
| 38 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 951.` |
| 39 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.822. Support: 227.` |
| 40 | `  •••start_col ≤ 43<br>	∧ -1.internal_type = Identifier<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.870. Support: 135.` |
| 41 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 15117.` |
| 42 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 7204.` |
| 43 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {MAP}<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.943. Support: 2855.` |
| 44 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {MAP}<br>	∧ -4.diff_offset ≥ 14<br>	∧ ^1.roles not in {IF, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.847. Support: 2102.` |
| 45 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 4191.` |
| 46 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 12<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.833. Support: 4105.` |
| 47 | `  -1.internal_type not in {Identifier, NumericLiteral, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -4.roles not in {KEY}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.853. Support: 3639.` |
| 48 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.945. Support: 2010.` |
| 49 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.920. Support: 2909.` |
| 50 | `  •••start_col ≤ 12<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +3.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.812. Support: 631.` |
| 51 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.internal_type = StringLiteral<br>	∧ -4.length ≥ 10<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.982. Support: 571.` |
| 52 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ -4.length ≤ 9<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.834. Support: 1044.` |
| 53 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 2236.` |
| 54 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 2315.` |
| 55 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.972. Support: 781.` |
| 56 | `  -1.internal_type not in {CommentLine, Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 888.` |
| 57 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 806.` |
| 58 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 131.` |
| 59 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = [<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +1.roles not in {KEY, LITERAL}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.806. Support: 286.` |
| 60 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -2.reserved = }<br>	∧ +1.reserved not in {), ,, ;, >, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +5.reserved = ,<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.865. Support: 352.` |
| 61 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -2.reserved not in {}}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.914. Support: 297.` |
| 62 | `  •••start_line ≥ 222<br>	∧ -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = function<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.808. Support: 486.` |
| 63 | `  •••start_col ≥ 5<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -2.reserved not in {}}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {), ,, ;, >, ], }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 16329.` |
| 64 | `  •••start_col ≥ 5<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -2.reserved not in {}}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.reserved not in {), ,, ;, >, ], }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {BLOCK, FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.880. Support: 3888.` |
| 65 | `  -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 22874.` |
| 66 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 4579.` |
| 67 | `  -1.reserved = {<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.902. Support: 2442.` |
| 68 | `  -1.reserved not in {(, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.948. Support: 1830.` |
| 69 | `  -1.reserved not in {(, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.917. Support: 2838.` |
| 70 | `  •••start_col ≥ 13<br>	∧ -1.reserved = ;<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.801. Support: 2357.` |
| 71 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.internal_type = StringLiteral<br>	∧ -4.length ≥ 10<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.975. Support: 585.` |
| 72 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -4.length ≤ 9<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {STRING}<br>	∧ +4.reserved = :<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.836. Support: 149.` |
| 73 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 835.` |
| 74 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.973. Support: 798.` |
| 75 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 140.` |
| 76 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -2.diff_offset ≤ 7<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved = =<br>	∧ ^1.internal_type not in {File, IfStatement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.897. Support: 151.` |
| 77 | `  -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.942. Support: 268.` |
| 78 | `  •••start_col ≥ 4<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 10972.` |
| 79 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = "<br>Confidence: 0.951. Support: 1994.` |
| 80 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {VALUE}<br>	∧ -4.roles in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = "<br>Confidence: 0.951. Support: 718.` |
| 81 | `  -1.internal_type = StringLiteral<br>	∧ -4.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = '<br>Confidence: 0.911. Support: 789.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -5.reserved = {<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.908. Support: 157.` |
| 83 | `  •••start_col ≤ 41<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {NUMBER}<br>	∧ -5.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.853. Support: 324.` |
| 84 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {NUMBER}<br>	∧ -5.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.863. Support: 3406.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 3890.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {LIST} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.847. Support: 449.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, LIST}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 296.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, LIST}<br>⇒ y = ∅<br>Confidence: 0.812. Support: 173.` |
| 89 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, LIST}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 3613.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.845. Support: 571.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.831. Support: 2077.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.912. Support: 1193.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 659.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 606.` |
| 95 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.899. Support: 442.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, function, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 26980.` |
| 97 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = "<br>Confidence: 0.948. Support: 2044.` |
| 98 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {MAP}<br>	∧ -4.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = "<br>Confidence: 0.935. Support: 700.` |
| 99 | `  •••start_line ≤ 164<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.label in {<space>}<br>	∧ -4.internal_type not in {StringLiteral}<br>	∧ +5.reserved = ,<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>	∧ ^2.roles not in {BINARY}<br>⇒ y = '<br>Confidence: 0.940. Support: 174.` |
| 100 | `  •••start_line ≤ 164<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.label in {<space>}<br>	∧ -4.internal_type not in {StringLiteral}<br>	∧ -4.length ≥ 7<br>	∧ +5.reserved not in {,}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>	∧ ^2.roles not in {BINARY}<br>⇒ y = '<br>Confidence: 0.816. Support: 160.` |
| 101 | `  -1.internal_type = StringLiteral<br>	∧ -2.label not in {<space>}<br>	∧ -4.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = '<br>Confidence: 0.840. Support: 1842.` |
| 102 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.827. Support: 1363.` |
| 103 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 4499.` |
| 104 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 8<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.875. Support: 3570.` |
| 105 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.945. Support: 1849.` |
| 106 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.922. Support: 2913.` |
| 107 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.roles in {STRING}<br>	∧ +1.roles in {VALUE} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.956. Support: 747.` |
| 108 | `  •••start_line ≥ 218<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.roles not in {STRING}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.828. Support: 854.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.997. Support: 918.` |
| 110 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 758.` |
| 111 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = IfStatement<br>⇒ y = ␣<br>Confidence: 0.990. Support: 154.` |
| 112 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = '<br>Confidence: 0.897. Support: 316.` |
| 113 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 11191.` |
| 114 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.roles in {KEY}<br>	∧ -5.reserved = {<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 158.` |
| 115 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.roles not in {KEY}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.841. Support: 3450.` |
| 116 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 322.` |
| 117 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.824. Support: 417.` |
| 118 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.844. Support: 176.` |
| 119 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 3528.` |
| 120 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.852. Support: 625.` |
| 121 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.908. Support: 1255.` |
| 122 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 611.` |
| 123 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 571.` |
| 124 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.897. Support: 433.` |
| 125 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, function, if}<br>	∧ -2.length ≤ 6<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 481.` |
| 126 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 22654.` |
| 127 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.949. Support: 2010.` |
| 128 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {MAP} and not in {KEY}<br>	∧ -4.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.953. Support: 738.` |
| 129 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.818. Support: 1380.` |
| 130 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 4696.` |
| 131 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 11<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.858. Support: 3568.` |
| 132 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.941. Support: 1910.` |
| 133 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.922. Support: 2940.` |
| 134 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.roles in {STRING}<br>	∧ -4.length ≥ 6<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.956. Support: 697.` |
| 135 | `  •••start_line ≥ 242<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.roles not in {STRING}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.820. Support: 838.` |
| 136 | `  •••start_line ≤ 241<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.roles not in {STRING}<br>	∧ -3.length ≥ 9<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.851. Support: 231.` |
| 137 | `  •••start_line ≤ 241<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.roles not in {STRING}<br>	∧ -3.length ≤ 8<br>	∧ -5.diff_col ≤ 13<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 6<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.824. Support: 173.` |
| 138 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 878.` |
| 139 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 143.` |
| 140 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -3.diff_offset ≤ 6<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, IfStatement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.835. Support: 227.` |
| 141 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.915. Support: 254.` |
| 142 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 10940.` |
| 143 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 320.` |
| 144 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -3.label in {<-space>}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.866. Support: 384.` |
| 145 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -3.label not in {<-space>}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.804. Support: 181.` |
| 146 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ -3.label not in {<-space>}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.916. Support: 3616.` |
| 147 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.809. Support: 2240.` |
| 148 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 230.` |
| 149 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 144.` |
| 150 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 625.` |
| 151 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 608.` |
| 152 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -2.length ≥ 10<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.905. Support: 142.` |
| 153 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, function, if}<br>	∧ -2.length ≤ 10<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 526.` |
| 154 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 26166.` |
| 155 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.831. Support: 1416.` |
| 156 | `  -1.reserved = {<br>	∧ -3.diff_col ≤ 12<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.860. Support: 3735.` |
| 157 | `  •••start_col ≥ 56<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_line = 0<br>	∧ -3.diff_offset ≥ 17<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 205.` |
| 158 | `  •••start_col ≤ 55<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 2604.` |
| 159 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.roles in {STRING}<br>	∧ -4.length ≥ 6<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.972. Support: 695.` |
| 160 | `  •••start_col ≤ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.835. Support: 191.` |
| 161 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.807. Support: 163.` |
| 162 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 11378.` |
| 163 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {MAP}<br>	∧ -4.roles in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = "<br>Confidence: 0.945. Support: 683.` |
| 164 | `  -1.internal_type = StringLiteral<br>	∧ -4.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {CALL} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = '<br>Confidence: 0.880. Support: 1261.` |
| 165 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {NUMBER}<br>	∧ -5.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.835. Support: 3851.` |
| 166 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.831. Support: 454.` |
| 167 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 322.` |
| 168 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.871. Support: 174.` |
| 169 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 3466.` |
| 170 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 952.` |
| 171 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 622.` |
| 172 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 616.` |
| 173 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.919. Support: 399.` |
| 174 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -2.length ≥ 11<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, VARIABLE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.868. Support: 133.` |
| 175 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, function, if}<br>	∧ -2.length ≤ 10<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 588.` |
| 176 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 26272.` |
| 177 | `  -1.internal_type = StringLiteral<br>	∧ +3.length ≤ 3<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.933. Support: 278.` |
| 178 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 22689.` |
| 179 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.810. Support: 1463.` |
| 180 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 4620.` |
| 181 | `  -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≤ 8<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.900. Support: 2922.` |
| 182 | `  •••start_col ≤ 13<br>	∧ -1.reserved = {<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_col ≤ 8<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.998. Support: 270.` |
| 183 | `  -1.reserved not in {(, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.946. Support: 1854.` |
| 184 | `  -1.reserved not in {(, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.927. Support: 2910.` |
| 185 | `  •••start_col ≤ 12<br>	∧ -1.reserved = ;<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.length ≥ 7<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.863. Support: 514.` |
| 186 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.internal_type = StringLiteral<br>	∧ +1.roles in {VALUE} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.959. Support: 679.` |
| 187 | `  •••start_line ≥ 218<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.832. Support: 832.` |
| 188 | `  •••start_line ≤ 217<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -3.length ≥ 9<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.854. Support: 189.` |
| 189 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 914.` |
| 190 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.973. Support: 695.` |
| 191 | `  •••start_col ≥ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -2.diff_line ≥ 1<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.871. Support: 151.` |
| 192 | `  •••start_col ≥ 8<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -2.diff_line = 0<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {SCOPE} and not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.888. Support: 886.` |
| 193 | `  •••start_col ≥ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -2.diff_line = 0<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, QUALIFIED, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 12000.` |
| 194 | `  •••start_col ≤ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type = Identifier<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 183.` |
| 195 | `  •••start_col ≤ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.903. Support: 149.` |
| 196 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {MAP}<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.949. Support: 2686.` |
| 197 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles not in {MAP}<br>	∧ -2.label not in {<space>}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.873. Support: 1773.` |
| 198 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {VALUE}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.844. Support: 3689.` |
| 199 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 3752.` |
| 200 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 310.` |
| 201 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {LIST, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 3563.` |
| 202 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.861. Support: 586.` |
| 203 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.820. Support: 2096.` |
| 204 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 922.` |
| 205 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 676.` |
| 206 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 587.` |
| 207 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.909. Support: 445.` |
| 208 | `  •••start_line ≥ 231<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = function<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.812. Support: 428.` |
| 209 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, function, if}<br>	∧ -2.length ≤ 6<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 559.` |
| 210 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 26425.` |
| 211 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {MAP}<br>	∧ -4.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = "<br>Confidence: 0.958. Support: 775.` |
| 212 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {CALL}<br>	∧ -4.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = '<br>Confidence: 0.949. Support: 581.` |
| 213 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.813. Support: 1439.` |
| 214 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 4589.` |
| 215 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.872. Support: 3087.` |
| 216 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_col ≤ 3<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.859. Support: 336.` |
| 217 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {SCOPE}<br>⇒ y = ⏎<br>Confidence: 0.813. Support: 2243.` |
| 218 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.internal_type = StringLiteral<br>	∧ -4.length ≥ 6<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.957. Support: 726.` |
| 219 | `  •••start_line ≥ 217<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.828. Support: 834.` |
| 220 | `  •••start_col ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -2.diff_line ≥ 1<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = '<br>Confidence: 0.859. Support: 153.` |
| 221 | `  •••start_col ≥ 8<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -2.diff_line = 0<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.921. Support: 827.` |
| 222 | `  •••start_col ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -2.diff_line = 0<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, File, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 12050.` |
| 223 | `  •••start_col ≤ 7<br>	∧ -1.diff_col ≥ 5<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = '<br>Confidence: 0.891. Support: 161.` |
| 224 | `  •••start_col ≤ 7<br>	∧ -1.diff_col ≤ 4<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 159.` |
| 225 | `  •••start_col ≥ 39<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {VALUE}<br>	∧ -5.reserved not in {;}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.872. Support: 808.` |
| 226 | `  •••start_col ≥ 39<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {VALUE}<br>	∧ -5.diff_col ≥ 18<br>	∧ -5.reserved not in {;}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.831. Support: 269.` |
| 227 | `  •••start_col ≤ 38<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {VALUE}<br>	∧ -5.reserved not in {;}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.917. Support: 2042.` |
| 228 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LIST}<br>⇒ y = ⏎<br>Confidence: 0.824. Support: 418.` |
| 229 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.823. Support: 2088.` |
| 230 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, function, if}<br>	∧ -2.length ≤ 6<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 453.` |
| 231 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 25871.` |
| 232 | `  -1.roles in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.841. Support: 431.` |
| 233 | `  -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.877. Support: 3243.` |
| 234 | `  -1.reserved = {<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_col ≤ 3<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.854. Support: 347.` |
| 235 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.internal_type = StringLiteral<br>	∧ +1.roles in {MAP} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.954. Support: 714.` |
| 236 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 925.` |
| 237 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.970. Support: 691.` |
| 238 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 154.` |
| 239 | `  -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.930. Support: 279.` |
| 240 | `  •••start_col ≥ 4<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 11068.` |
| 241 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = "<br>Confidence: 0.950. Support: 1982.` |
| 242 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {VALUE}<br>	∧ -4.roles in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = "<br>Confidence: 0.956. Support: 690.` |
| 243 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {CALL}<br>	∧ -4.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = '<br>Confidence: 0.949. Support: 574.` |
| 244 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.label in {<-space>}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.886. Support: 380.` |
| 245 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -3.label not in {<-space>}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 317.` |
| 246 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ -3.label not in {<-space>}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 3516.` |
| 247 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.889. Support: 1223.` |
| 248 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 257.` |
| 249 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.863. Support: 142.` |
| 250 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 646.` |
| 251 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 549.` |
| 252 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.882. Support: 429.` |
| 253 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, function, if}<br>	∧ -2.length ≤ 6<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 433.` |
| 254 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 26287.` |
| 255 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = "<br>Confidence: 0.948. Support: 2040.` |
| 256 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {MAP}<br>	∧ -4.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = "<br>Confidence: 0.948. Support: 699.` |
| 257 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {CALL}<br>	∧ -4.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = '<br>Confidence: 0.962. Support: 589.` |
| 258 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.900. Support: 2481.` |
| 259 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.804. Support: 2402.` |
| 260 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.roles in {STRING}<br>	∧ -4.length ≥ 10<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.987. Support: 583.` |
| 261 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ -4.length ≤ 9<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.821. Support: 872.` |
| 262 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -2.diff_offset ≤ 4<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {BLOCK} and not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.837. Support: 199.` |
| 263 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.841. Support: 167.` |
| 264 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 10966.` |
| 265 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {VALUE}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.853. Support: 3778.` |
| 266 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.reserved = }<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.808. Support: 483.` |
| 267 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 3604.` |
| 268 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -2.length ≥ 10<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.940. Support: 158.` |
| 269 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, function, if}<br>	∧ -2.length ≤ 10<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 515.` |
| 270 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 25987.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.9185185185185185, "max_conf": 0.9994369149208069, "max_support": 26980, "min_conf": 0.8008061051368713, "min_support": 131, "num_rules": 270}}
```
</details>
