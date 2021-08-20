# Model report for file:///tmp/top-repos-quality-repos-971ma9tt/detect-avoid.git HEAD 79d8021b942b6746e42b12039dfc04d30cb55e2e

### Dump

```json
{'created_at': '2021-08-19 16:05:37',
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
 'size': '27.6 kB',
 'tags': [],
 'uuid': 'fad0767d-df6f-4715-9c52-beaeb800d45e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-971ma9tt/detect-avoid.git 79d8021b942b6746e42b12039dfc04d30cb55e2e

# javascript
103 rules, avg.len. 10.5
## train
PPCR: 0.922128
### report
macro
{'f1-score': 0.49819362016338975,
 'precision': 0.5741606166326382,
 'recall': 0.46563915482448726,
 'support': 386902}
micro
{'f1-score': 0.9567849222800606,
 'precision': 0.9567849222800606,
 'recall': 0.9567849222800606,
 'support': 386902}
weighted
{'f1-score': 0.9538783287563282,
 'precision': 0.952908708650626,
 'recall': 0.9567849222800606,
 'support': 386902}
### report_full
macro
{'f1-score': 0.42627551822428617,
 'precision': 0.5741606166326382,
 'recall': 0.37611521152336524,
 'support': 419575}
micro
{'f1-score': 0.9180224606529386,
 'precision': 0.9567849222800606,
 'recall': 0.8822784960972413,
 'support': 419575}
weighted
{'f1-score': 0.9059926063790387,
 'precision': 0.9453626020546353,
 'recall': 0.8822784960972413,
 'support': 419575}
## test
PPCR: 0.924193
### report
macro
{'f1-score': 0.483660711397098,
 'precision': 0.5695778037680647,
 'recall': 0.4580806787449803,
 'support': 86059}
micro
{'f1-score': 0.9554724084639608,
 'precision': 0.9554724084639608,
 'recall': 0.9554724084639608,
 'support': 86059}
weighted
{'f1-score': 0.9521282669146193,
 'precision': 0.9511846942472529,
 'recall': 0.9554724084639608,
 'support': 86059}
### report_full
macro
{'f1-score': 0.420341248270496,
 'precision': 0.5695778037680647,
 'recall': 0.37374480273262356,
 'support': 93118}
micro
{'f1-score': 0.9178298553943866,
 'precision': 0.9554724084639608,
 'recall': 0.8830408728709809,
 'support': 93118}
weighted
{'f1-score': 0.9063537029741349,
 'precision': 0.9453525173805806,
 'recall': 0.8830408728709809,
 'support': 93118}
```

## javascript
### Summary
68 rules, avg.len. 10.4

| | |
|-|-|
|Min support|94|
|Max support|39425|
|Min confidence|0.920648455619812|
|Max confidence|0.9990925788879395|

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
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.982. Support: 570.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.996. Support: 3365.` |
| 3 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.999. Support: 1342.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.995. Support: 110.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ]<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.984. Support: 159.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ]<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 981.` |
| 7 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.988. Support: 563.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.997. Support: 174.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 112.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, {, ||}<br>	∧ -2.diff_offset ≤ 9<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {(, )}<br>	∧ +2.reserved = .<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 2802.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, {, ||}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {(, )}<br>	∧ +2.reserved not in {.}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 23206.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {[, {}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.reserved = &&<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {AND, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 155.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.roles in {AND, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 157.` |
| 14 | `  •••start_col ≤ 73<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -5.roles in {LEFT}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.length ≤ 17<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 182.` |
| 15 | `  •••start_col ≤ 73<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.length ≤ 17<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 249.` |
| 16 | `  •••start_col ≤ 73<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.length ≤ 17<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 5861.` |
| 17 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.991. Support: 6262.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 6454.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), =}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 1343.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION, IF}<br>	∧ +1.reserved not in {), =}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 126.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 166.` |
| 22 | `  -1.internal_type = DirectiveLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.997. Support: 168.` |
| 23 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 209.` |
| 24 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 266.` |
| 25 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.diff_line = 0<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ;<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 3487.` |
| 26 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved = (<br>	∧ -5.diff_line = 0<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles in {IF} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 116.` |
| 27 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved not in {(}<br>	∧ -5.diff_line = 0<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 6495.` |
| 28 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), =, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 39425.` |
| 29 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 2180.` |
| 30 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 10361.` |
| 31 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-tab>}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.982. Support: 139.` |
| 32 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-tab>}<br>	∧ -4.label in {<-tab>}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.973. Support: 94.` |
| 33 | `  -1.diff_col ≤ 7<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-tab>}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.934. Support: 3694.` |
| 34 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.990. Support: 3690.` |
| 35 | `  -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.997. Support: 168.` |
| 36 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.934. Support: 5340.` |
| 37 | `  -1.diff_col ≥ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 1612.` |
| 38 | `  -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = this<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type = VariableDeclaration<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.946. Support: 342.` |
| 39 | `  -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {this}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type = VariableDeclaration<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.964. Support: 1126.` |
| 40 | `  -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles in {SCOPE, STATEMENT} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 280.` |
| 41 | `  -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 586.` |
| 42 | `  -1.diff_col ≥ 9<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = '<br>Confidence: 0.999. Support: 551.` |
| 43 | `  -1.diff_col ≥ 9<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.979. Support: 2159.` |
| 44 | `  -1.diff_col ≤ 8<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = '<br>Confidence: 0.999. Support: 375.` |
| 45 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = (<br>	∧ -5.diff_offset ≥ 27<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 289.` |
| 46 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -5.diff_offset ≤ 26<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 2986.` |
| 47 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -5.diff_offset ≤ 26<br>	∧ -5.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 647.` |
| 48 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, [, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 159.` |
| 49 | `  -1.diff_col ≤ 8<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, [, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 546.` |
| 50 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, [, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {CallExpression, File, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 12879.` |
| 51 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = -<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 168.` |
| 52 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 9718.` |
| 53 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ -5.diff_offset ≥ 17<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 694.` |
| 54 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 5986.` |
| 55 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 23<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.978. Support: 1179.` |
| 56 | `  •••start_col ≤ 22<br>	∧ -1.reserved not in {(, ,, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 22<br>	∧ -3.diff_col ≥ 1<br>	∧ -5.diff_offset ≥ 231<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {COMMENT}<br>	∧ +2.length ≥ 8<br>	∧ +5.reserved not in {.}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.955. Support: 431.` |
| 57 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = function<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved = (<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 1642.` |
| 58 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved = (<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 1093.` |
| 59 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = else<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved = (<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 279.` |
| 60 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved = ,<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved = (<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.975. Support: 141.` |
| 61 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 772.` |
| 62 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 758.` |
| 63 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {KEY}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 403.` |
| 64 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {KEY}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {(, else, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 327.` |
| 65 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {COMMENT} and not in {KEY}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 231.` |
| 66 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {COMMENT}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 214.` |
| 67 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved = return<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 100.` |
| 68 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved not in {(, ,, ;, =, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {COMMENT, KEY}<br>	∧ +1.reserved not in {(, ), {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 25198.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.382352941176471, "max_conf": 0.9990925788879395, "max_support": 39425, "min_conf": 0.920648455619812, "min_support": 94, "num_rules": 68}}
```
</details>
