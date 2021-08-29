# Model report for file:///tmp/top-repos-quality-repos-kh8l8bue/autoprefixer.git HEAD b5b5f5d01c03923d2750f827421b0f4db4b5e1e1

### Dump

```json
{'created_at': '2021-08-29 10:38:46',
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
 'size': '19.3 kB',
 'tags': [],
 'uuid': '40c41710-d9f4-419f-9832-0aa4b00a7f2b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-kh8l8bue/autoprefixer.git b5b5f5d01c03923d2750f827421b0f4db4b5e1e1

# javascript
175 rules, avg.len. 10.3
## train
PPCR: 0.980641
### report
macro
{'f1-score': 0.8918215768568399,
 'precision': 0.91478230786659,
 'recall': 0.8718669030117407,
 'support': 36370}
micro
{'f1-score': 0.9407478691229035,
 'precision': 0.9407478691229035,
 'recall': 0.9407478691229035,
 'support': 36370}
weighted
{'f1-score': 0.9400007917767581,
 'precision': 0.9400250103859732,
 'recall': 0.9407478691229035,
 'support': 36370}
### report_full
macro
{'f1-score': 0.867170834720123,
 'precision': 0.91478230786659,
 'recall': 0.8305738486297936,
 'support': 37088}
micro
{'f1-score': 0.9315527240055542,
 'precision': 0.9407478691229035,
 'recall': 0.9225355910267472,
 'support': 37088}
weighted
{'f1-score': 0.9294201661372108,
 'precision': 0.9391998395095965,
 'recall': 0.9225355910267472,
 'support': 37088}
## test
PPCR: 0.974100
### report
macro
{'f1-score': 0.9014612253020278,
 'precision': 0.9216071919934944,
 'recall': 0.8868175335546354,
 'support': 7146}
micro
{'f1-score': 0.9465435208508256,
 'precision': 0.9465435208508256,
 'recall': 0.9465435208508256,
 'support': 7146}
weighted
{'f1-score': 0.9459967841675684,
 'precision': 0.9464572880878268,
 'recall': 0.9465435208508256,
 'support': 7146}
### report_full
macro
{'f1-score': 0.8673515471035796,
 'precision': 0.9216071919934944,
 'recall': 0.8316846670877338,
 'support': 7336}
micro
{'f1-score': 0.934125120839663,
 'precision': 0.9465435208508256,
 'recall': 0.9220283533260633,
 'support': 7336}
weighted
{'f1-score': 0.9309977006473366,
 'precision': 0.9445161619854341,
 'recall': 0.9220283533260633,
 'support': 7336}
```

## javascript
### Summary
126 rules, avg.len. 10.1

| | |
|-|-|
|Min support|134|
|Max support|5326|
|Min confidence|0.9203821420669556|
|Max confidence|0.9997057318687439|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 5326.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 1.000. Support: 1632.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.979. Support: 170.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 273.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 148.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.964. Support: 1681.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.923. Support: 1181.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 491.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 3534.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 1586.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 700.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {BLOCK} and not in {IDENTIFIER}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 483.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 844.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 261.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 2337.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 459.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 336.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 236.` |
| 19 | `  -1.diff_col ≥ 12<br>	∧ -1.internal_type = CommentBlock<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 159.` |
| 20 | `  •••start_col ≥ 16<br>	∧ -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 396.` |
| 21 | `  •••start_col ≤ 15<br>	∧ -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 889.` |
| 22 | `  -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles in {MAP} and not in {COMMENT}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.920. Support: 157.` |
| 23 | `  -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 143.` |
| 24 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.969. Support: 5235.` |
| 25 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 1.000. Support: 1699.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 170.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 257.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 155.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.962. Support: 1784.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 2098.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 988.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 4069.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 315.` |
| 34 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 582.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 190.` |
| 36 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles in {MODULE} and not in {LITERAL, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 215.` |
| 37 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +3.length ≥ 5<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles in {MODULE} and not in {LITERAL, SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.978. Support: 207.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 284.` |
| 39 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 432.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER, SCOPE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 847.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER, SCOPE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 264.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER, SCOPE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 2422.` |
| 43 | `  •••start_col ≥ 19<br>	∧ -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 345.` |
| 44 | `  •••start_col ≥ 19<br>	∧ -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION, IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 336.` |
| 45 | `  •••start_col ≤ 18<br>	∧ -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 1102.` |
| 46 | `  -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +4.reserved = ,<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.997. Support: 145.` |
| 47 | `  -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 141.` |
| 48 | `  -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {CallExpression, ObjectExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 156.` |
| 49 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 5284.` |
| 50 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 1.000. Support: 1673.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.978. Support: 160.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 297.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.926. Support: 1137.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 514.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 3676.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 663.` |
| 57 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {SCOPE} and not in {QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 491.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED, SCOPE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 819.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED, SCOPE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 259.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED, SCOPE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 2348.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 466.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 274.` |
| 63 | `  -1.diff_col ≥ 14<br>	∧ -1.internal_type = CommentBlock<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 145.` |
| 64 | `  •••start_col ≥ 26<br>	∧ -1.diff_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 283.` |
| 65 | `  •••start_col ≥ 26<br>	∧ -1.diff_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 229.` |
| 66 | `  -1.diff_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.963. Support: 147.` |
| 67 | `  -1.diff_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {CallExpression, ObjectExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 165.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 259.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 2108.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 943.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 4228.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 322.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 213.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, let, }}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 619.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, let, }}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {MODULE} and not in {IDENTIFIER, LITERAL, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 217.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, let, }}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, MODULE, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 4990.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 136.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 2100.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 993.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 4104.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 297.` |
| 82 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.975. Support: 223.` |
| 83 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.997. Support: 189.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 318.` |
| 85 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {SCOPE} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 635.` |
| 86 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED, SCOPE}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 188.` |
| 87 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +3.length ≥ 5<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED, SCOPE}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ⏎⏎<br>Confidence: 0.989. Support: 220.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 274.` |
| 89 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED, SCOPE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 860.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED, SCOPE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 250.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED, SCOPE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 2245.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 255.` |
| 93 | `  -1.internal_type = CommentBlock<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type = File<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 160.` |
| 94 | `  -1.internal_type not in {CommentBlock, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 13<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 293.` |
| 95 | `  •••start_col ≥ 19<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type not in {CallExpression, File}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 444.` |
| 96 | `  •••start_col ≤ 18<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type not in {CallExpression, File}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 974.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +4.reserved = ,<br>	∧ ^1.internal_type not in {CallExpression, File}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.989. Support: 134.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {CallExpression, File}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 147.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 259.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {EXPRESSION} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.972. Support: 1509.` |
| 101 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {EXPRESSION, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 157.` |
| 102 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 4324.` |
| 103 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles in {COMMENT}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 280.` |
| 104 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {BLOCK} and not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 137.` |
| 105 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {BLOCK, EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.970. Support: 247.` |
| 106 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 301.` |
| 107 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 214.` |
| 108 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ ^1.roles in {SCOPE} and not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 657.` |
| 109 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ ^1.roles not in {FILE, QUALIFIED, SCOPE}<br>	∧ ^2.internal_type = File<br>⇒ y = ␣<br>Confidence: 0.997. Support: 199.` |
| 110 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +3.length ≥ 5<br>	∧ ^1.roles not in {FILE, QUALIFIED, SCOPE}<br>	∧ ^2.internal_type = File<br>⇒ y = ⏎⏎<br>Confidence: 0.975. Support: 217.` |
| 111 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = CallExpression<br>⇒ y = ␣<br>Confidence: 0.920. Support: 509.` |
| 112 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = CallExpression<br>⇒ y = ∅<br>Confidence: 0.967. Support: 3607.` |
| 113 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 678.` |
| 114 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 498.` |
| 115 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {BlockStatement, CallExpression, MemberExpression}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 843.` |
| 116 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, CallExpression, MemberExpression}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 242.` |
| 117 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {BlockStatement, CallExpression, MemberExpression}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 2128.` |
| 118 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 459.` |
| 119 | `  •••start_col ≥ 18<br>	∧ -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 379.` |
| 120 | `  •••start_col ≥ 18<br>	∧ -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 389.` |
| 121 | `  •••start_col ≤ 17<br>	∧ -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 950.` |
| 122 | `  -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 150.` |
| 123 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 267.` |
| 124 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 524.` |
| 125 | `  -1.diff_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +4.reserved = ,<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.977. Support: 150.` |
| 126 | `  -1.diff_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 162.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.134920634920634, "max_conf": 0.9997057318687439, "max_support": 5326, "min_conf": 0.9203821420669556, "min_support": 134, "num_rules": 126}}
```
</details>
