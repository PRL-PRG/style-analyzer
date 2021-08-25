# Model report for file:///tmp/top-repos-quality-repos-ke1ue5c3/demo.git HEAD 2511ba1002cf1dc3be40d2ad647e428c48802bc0

### Dump

```json
{'created_at': '2021-08-24 20:32:16',
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
 'size': '22.4 kB',
 'tags': [],
 'uuid': '510ac137-4440-40ab-aa56-1fd7d8796352',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ke1ue5c3/demo.git 2511ba1002cf1dc3be40d2ad647e428c48802bc0

# javascript
45 rules, avg.len. 9.2
## train
PPCR: 0.826262
### report
macro
{'f1-score': 0.7016591198353428,
 'precision': 0.7679479479692458,
 'recall': 0.662253489704132,
 'support': 43544}
micro
{'f1-score': 0.9451129891603894,
 'precision': 0.9451129891603894,
 'recall': 0.9451129891603894,
 'support': 43544}
weighted
{'f1-score': 0.9425089456473832,
 'precision': 0.9431009026063191,
 'recall': 0.9451129891603894,
 'support': 43544}
### report_full
macro
{'f1-score': 0.548495606532943,
 'precision': 0.7679479479692458,
 'recall': 0.45659166727278166,
 'support': 52700}
micro
{'f1-score': 0.8552013632018619,
 'precision': 0.9451129891603894,
 'recall': 0.7809108159392789,
 'support': 52700}
weighted
{'f1-score': 0.8406341225685139,
 'precision': 0.9367274772813103,
 'recall': 0.7809108159392789,
 'support': 52700}
## test
PPCR: 0.841121
### report
macro
{'f1-score': 0.7074120856683833,
 'precision': 0.7772058860655847,
 'recall': 0.6675567036848049,
 'support': 11647}
micro
{'f1-score': 0.9458229587018117,
 'precision': 0.9458229587018117,
 'recall': 0.9458229587018117,
 'support': 11647}
weighted
{'f1-score': 0.9433912635106615,
 'precision': 0.9444021013447189,
 'recall': 0.9458229587018117,
 'support': 11647}
### report_full
macro
{'f1-score': 0.5541655042000889,
 'precision': 0.7772058860655847,
 'recall': 0.46331672097508453,
 'support': 13847}
micro
{'f1-score': 0.8642033419628149,
 'precision': 0.9458229587018117,
 'recall': 0.7955513829710407,
 'support': 13847}
weighted
{'f1-score': 0.8499454023548195,
 'precision': 0.9393498486091015,
 'recall': 0.7955513829710407,
 'support': 13847}
```

## javascript
### Summary
28 rules, avg.len. 8.4

| | |
|-|-|
|Min support|98|
|Max support|12407|
|Min confidence|0.9233668446540833|
|Max confidence|0.9995408654212952|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>⇒ y = "<br>Confidence: 0.995. Support: 108.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>⇒ y = '<br>Confidence: 0.954. Support: 2001.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.980. Support: 1044.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {EXPRESSION}<br>⇒ y = '<br>Confidence: 0.929. Support: 489.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {EXPRESSION}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.934. Support: 128.` |
| 6 | `  •••start_col ≥ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {EXPRESSION}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 605.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 2104.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1894.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1089.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 966.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.length ≤ 2<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 340.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<space>}<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {IMPORT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 111.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 806.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type = Identifier<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 117.` |
| 15 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression, JSXElement, ObjectExpression}<br>	∧ ^1.roles in {STATEMENT} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.941. Support: 282.` |
| 16 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression, JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 190.` |
| 17 | `  •••start_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 2<br>	∧ -2.diff_col = 0<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 196.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 959.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-tab>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, VARIABLE}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.997. Support: 185.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, VARIABLE}<br>	∧ ^2.internal_type not in {AssignmentExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.923. Support: 398.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {BINARY}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles in {OPERATOR} and not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 98.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 633.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {LIST} and not in {INCOMPLETE, OPERATOR, VARIABLE}<br>⇒ y = ⏎<br>Confidence: 0.948. Support: 144.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, LIST, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 177.` |
| 25 | `  •••start_col ≥ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 113.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 114.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if, {}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +3.internal_type = JSXIdentifier<br>	∧ ^1.roles in {INCOMPLETE} and not in {CONDITION, OPERATOR, VARIABLE}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 501.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if, {}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 12407.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.428571428571429, "max_conf": 0.9995408654212952, "max_support": 12407, "min_conf": 0.9233668446540833, "min_support": 98, "num_rules": 28}}
```
</details>
