# Model report for file:///tmp/top-repos-quality-repos-9dd21pez/engine.io.git HEAD 7e9e544a9ac4365ef055c9482cc1e9fed624031c

### Dump

```json
{'created_at': '2021-08-30 18:44:50',
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
 'size': '19.0 kB',
 'tags': [],
 'uuid': 'f3a64abe-1394-449a-b7c1-dfb98abc817e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-9dd21pez/engine.io.git 7e9e544a9ac4365ef055c9482cc1e9fed624031c

# javascript
117 rules, avg.len. 6.6
## train
PPCR: 0.997764
### report
macro
{'f1-score': 0.7381806886704565,
 'precision': 0.771929796824327,
 'recall': 0.715964769358458,
 'support': 31681}
micro
{'f1-score': 0.9236766516208453,
 'precision': 0.9236766516208453,
 'recall': 0.9236766516208453,
 'support': 31681}
weighted
{'f1-score': 0.9189456579172756,
 'precision': 0.9178920669622119,
 'recall': 0.9236766516208453,
 'support': 31681}
### report_full
macro
{'f1-score': 0.7329539257127633,
 'precision': 0.771929796824327,
 'recall': 0.7088073000545909,
 'support': 31752}
micro
{'f1-score': 0.9226427884539593,
 'precision': 0.9236766516208453,
 'recall': 0.9216112370874275,
 'support': 31752}
weighted
{'f1-score': 0.9174520757066728,
 'precision': 0.9175827807858811,
 'recall': 0.9216112370874275,
 'support': 31752}
## test
PPCR: 0.994093
### report
macro
{'f1-score': 0.709005723064275,
 'precision': 0.7713388003848378,
 'recall': 0.6878873310735669,
 'support': 6732}
micro
{'f1-score': 0.9013666072489602,
 'precision': 0.9013666072489602,
 'recall': 0.9013666072489602,
 'support': 6732}
weighted
{'f1-score': 0.890460230533482,
 'precision': 0.8911055127991715,
 'recall': 0.9013666072489602,
 'support': 6732}
### report_full
macro
{'f1-score': 0.7015223482687929,
 'precision': 0.7713388003848378,
 'recall': 0.6813116519295674,
 'support': 6772}
micro
{'f1-score': 0.898696682464455,
 'precision': 0.9013666072489602,
 'recall': 0.896042528056704,
 'support': 6772}
weighted
{'f1-score': 0.8858565859510377,
 'precision': 0.8909167443169,
 'recall': 0.896042528056704,
 'support': 6772}
```

## javascript
### Summary
79 rules, avg.len. 6.7

| | |
|-|-|
|Min support|129|
|Max support|13239|
|Min confidence|0.9210015535354614|
|Max confidence|0.9996031522750854|

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
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.965. Support: 1142.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.970. Support: 4688.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.959. Support: 765.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 949.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {VALUE}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.967. Support: 893.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 12<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.923. Support: 150.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1246.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {STRING}<br>	∧ +4.reserved not in {>}<br>	∧ ^1.internal_type not in {BinaryExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 494.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {STRING}<br>	∧ +4.reserved not in {>}<br>	∧ ^1.internal_type not in {BinaryExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 1534.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.length ≤ 1<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.internal_type not in {BinaryExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 5403.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.995. Support: 776.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 171.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 8045.` |
| 14 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.996. Support: 129.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 4752.` |
| 16 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 703.` |
| 17 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.962. Support: 797.` |
| 18 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.953. Support: 904.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.993. Support: 804.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1236.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 1018.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 2576.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 162.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.diff_col ≥ 11<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 887.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 11<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 233.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.diff_col ≤ 10<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 6390.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.960. Support: 731.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 897.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.996. Support: 796.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1255.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.diff_col ≥ 11<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 970.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 11<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 246.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_col ≤ 10<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 6433.` |
| 34 | `  -1.roles in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = "<br>Confidence: 0.997. Support: 164.` |
| 35 | `  -1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.969. Support: 4633.` |
| 36 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 900.` |
| 37 | `  -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {:}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.998. Support: 800.` |
| 38 | `  -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 176.` |
| 39 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1260.` |
| 40 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.946. Support: 934.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.diff_col ≥ 11<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.internal_type not in {BinaryExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 888.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 11<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.internal_type not in {BinaryExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 224.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_col ≤ 10<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.internal_type not in {BinaryExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 6478.` |
| 44 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.957. Support: 799.` |
| 45 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1003.` |
| 46 | `  -1.reserved = {<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +3.roles not in {VALUE}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.952. Support: 599.` |
| 47 | `  -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY, STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 2618.` |
| 48 | `  +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1235.` |
| 49 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.947. Support: 939.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.992. Support: 298.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 172.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.994. Support: 804.` |
| 53 | `  •••start_col ≤ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.921. Support: 197.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 165.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.diff_col ≥ 11<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 930.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 11<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 257.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_col ≤ 10<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 6437.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.954. Support: 757.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 937.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.994. Support: 792.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 154.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.959. Support: 836.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 697.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.947. Support: 516.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = {<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 574.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION} and not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 2180.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 1035.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.982. Support: 311.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 374.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 237.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 175.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 145.` |
| 73 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 12449.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 493.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {>}<br>	∧ ^1.roles in {DECLARATION} and not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 2187.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 984.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.988. Support: 297.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 257.` |
| 79 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 13239.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.69620253164557, "max_conf": 0.9996031522750854, "max_support": 13239, "min_conf": 0.9210015535354614, "min_support": 129, "num_rules": 79}}
```
</details>
