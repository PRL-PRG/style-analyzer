# Model report for file:///tmp/top-repos-quality-repos-ip41hahb/tournament-organiser.git HEAD 5affa81dfbe6697d79972f0013595f7efcfbe9ea

### Dump

```json
{'created_at': '2021-08-24 22:51:54',
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
 'size': '20.5 kB',
 'tags': [],
 'uuid': '84474185-a0d8-4dbc-95d7-ec6f2e67173f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ip41hahb/tournament-organiser.git 5affa81dfbe6697d79972f0013595f7efcfbe9ea

# javascript
90 rules, avg.len. 6.9
## train
PPCR: 0.927038
### report
macro
{'f1-score': 0.4281849999488778,
 'precision': 0.44395555220700267,
 'recall': 0.41676339654744543,
 'support': 28334}
micro
{'f1-score': 0.9209430366344321,
 'precision': 0.9209430366344321,
 'recall': 0.9209430366344321,
 'support': 28334}
weighted
{'f1-score': 0.9098127381579066,
 'precision': 0.9008001883400687,
 'recall': 0.9209430366344321,
 'support': 28334}
### report_full
macro
{'f1-score': 0.39418449176957093,
 'precision': 0.44395555220700267,
 'recall': 0.3720926446446377,
 'support': 30564}
micro
{'f1-score': 0.8860742300247885,
 'precision': 0.9209430366344321,
 'recall': 0.853749509226541,
 'support': 30564}
weighted
{'f1-score': 0.8558663764599804,
 'precision': 0.8736300162265821,
 'recall': 0.853749509226541,
 'support': 30564}
## test
PPCR: 0.920686
### report
macro
{'f1-score': 0.4265221452917013,
 'precision': 0.44125441716715014,
 'recall': 0.4163268197159722,
 'support': 7139}
micro
{'f1-score': 0.9242190783022832,
 'precision': 0.9242190783022832,
 'recall': 0.9242190783022832,
 'support': 7139}
weighted
{'f1-score': 0.9138613477079095,
 'precision': 0.9056966270628074,
 'recall': 0.9242190783022832,
 'support': 7139}
### report_full
macro
{'f1-score': 0.392260121571827,
 'precision': 0.44125441716715014,
 'recall': 0.36897807417852646,
 'support': 7754}
micro
{'f1-score': 0.8860538508023903,
 'precision': 0.9242190783022832,
 'recall': 0.8509156564353881,
 'support': 7754}
weighted
{'f1-score': 0.8540862836767587,
 'precision': 0.8721764750226007,
 'recall': 0.8509156564353881,
 'support': 7754}
```

## javascript
### Summary
49 rules, avg.len. 6.8

| | |
|-|-|
|Min support|137|
|Max support|12679|
|Min confidence|0.9203910827636719|
|Max confidence|0.9980769157409668|

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
| 1 | `  -1.roles in {STRING}<br>⇒ y = "<br>Confidence: 0.945. Support: 1582.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {LITERAL, STRING}<br>⇒ y = "<br>Confidence: 0.967. Support: 583.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {LITERAL} and not in {STRING}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 174.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≤ 4<br>	∧ +1.roles in {LITERAL}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 1004.` |
| 5 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 805.` |
| 6 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 418.` |
| 7 | `  -1.reserved = )<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 333.` |
| 8 | `  -1.reserved = )<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {., {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 745.` |
| 9 | `  -1.diff_col ≥ 3<br>	∧ -1.reserved not in {), ,, :, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 259.` |
| 10 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 232.` |
| 11 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ +3.roles not in {VALUE}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.920. Support: 358.` |
| 12 | `  -1.reserved not in {), ,, :, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 12679.` |
| 13 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.954. Support: 593.` |
| 14 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 188.` |
| 15 | `  -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.977. Support: 151.` |
| 16 | `  -1.reserved not in {), ,, :, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 11355.` |
| 17 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.diff_line ≥ 1<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.989. Support: 137.` |
| 18 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_line = 0<br>	∧ +1.roles in {LITERAL}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 1055.` |
| 19 | `  -1.reserved not in {), ,, :, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 11930.` |
| 20 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.941. Support: 1556.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {LITERAL, STRING}<br>⇒ y = "<br>Confidence: 0.969. Support: 558.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {LITERAL} and not in {STRING}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 192.` |
| 23 | `  -1.diff_offset ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.923. Support: 137.` |
| 24 | `  -1.diff_offset ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 1062.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 766.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 461.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 344.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.reserved not in {., {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 793.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 218.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 11188.` |
| 31 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 434.` |
| 32 | `  -1.reserved = )<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {., {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 826.` |
| 33 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 219.` |
| 34 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +3.roles not in {VALUE}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.940. Support: 356.` |
| 35 | `  -1.reserved not in {), ,, :, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≥ 3<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 228.` |
| 36 | `  -1.reserved not in {), ,, :, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 11077.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≤ 4<br>	∧ +1.roles in {LITERAL}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 1073.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 446.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.reserved not in {., {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 816.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.reserved = )<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.993. Support: 206.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 176.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 11209.` |
| 43 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 1019.` |
| 44 | `  -1.diff_offset ≥ 3<br>	∧ -1.reserved not in {), ,, :, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 260.` |
| 45 | `  -1.reserved not in {), ,, :, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 11092.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 11288.` |
| 47 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 197.` |
| 48 | `  -1.reserved not in {), ,, :, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≥ 2<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 235.` |
| 49 | `  -1.diff_offset ≤ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 1055.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.836734693877551, "max_conf": 0.9980769157409668, "max_support": 12679, "min_conf": 0.9203910827636719, "min_support": 137, "num_rules": 49}}
```
</details>
