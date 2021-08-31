# Model report for file:///tmp/top-repos-quality-repos-7hsbvtpn/lbry-desktop.git HEAD 79be67831b67ab52a0b04d8cb224131b072a86da

### Dump

```json
{'created_at': '2021-08-30 18:03:19',
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
 'size': '24.2 kB',
 'tags': [],
 'uuid': '2d8df5d9-913b-49d6-a76a-c2c80d98c458',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-7hsbvtpn/lbry-desktop.git 79be67831b67ab52a0b04d8cb224131b072a86da

# javascript
344 rules, avg.len. 9.8
## train
PPCR: 0.971150
### report
macro
{'f1-score': 0.7034752815932063,
 'precision': 0.7241368395101211,
 'recall': 0.6854175905467337,
 'support': 174671}
micro
{'f1-score': 0.9560430752672167,
 'precision': 0.9560430752672167,
 'recall': 0.9560430752672167,
 'support': 174671}
weighted
{'f1-score': 0.954309792327232,
 'precision': 0.9536142618686215,
 'recall': 0.9560430752672167,
 'support': 174671}
### report_full
macro
{'f1-score': 0.6672295749251356,
 'precision': 0.7241368395101211,
 'recall': 0.6269537229688105,
 'support': 179860}
micro
{'f1-score': 0.9420502015338574,
 'precision': 0.9560430752672167,
 'recall': 0.9284610252418548,
 'support': 179860}
weighted
{'f1-score': 0.9382288644459457,
 'precision': 0.9522111504801644,
 'recall': 0.9284610252418548,
 'support': 179860}
## test
PPCR: 0.978402
### report
macro
{'f1-score': 0.7129923705381488,
 'precision': 0.7307505924266486,
 'recall': 0.6975046357503284,
 'support': 12095}
micro
{'f1-score': 0.9634559735427862,
 'precision': 0.9634559735427862,
 'recall': 0.9634559735427862,
 'support': 12095}
weighted
{'f1-score': 0.962463735095108,
 'precision': 0.962001219933899,
 'recall': 0.9634559735427862,
 'support': 12095}
### report_full
macro
{'f1-score': 0.6787527203803209,
 'precision': 0.7307505924266486,
 'recall': 0.6477609175117005,
 'support': 12362}
micro
{'f1-score': 0.9529378092161753,
 'precision': 0.9634559735427862,
 'recall': 0.9426468209027665,
 'support': 12362}
weighted
{'f1-score': 0.9489692048118177,
 'precision': 0.9600619443301374,
 'recall': 0.9426468209027665,
 'support': 12362}
```

## javascript
### Summary
255 rules, avg.len. 9.5

| | |
|-|-|
|Min support|132|
|Max support|38463|
|Min confidence|0.920051097869873|
|Max confidence|0.9998957514762878|

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
| 1 | `  +1.reserved not in {}}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.980. Support: 16914.` |
| 2 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.978. Support: 575.` |
| 3 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 6461.` |
| 4 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 1.000. Support: 3071.` |
| 5 | `  •••start_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {FUNCTION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⏎<br>Confidence: 0.940. Support: 176.` |
| 6 | `  •••start_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {FUNCTION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.980. Support: 1011.` |
| 7 | `  •••start_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {import}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {LITERAL} and not in {FUNCTION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.953. Support: 652.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved = ,<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 168.` |
| 9 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {,}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.929. Support: 3649.` |
| 10 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.994. Support: 764.` |
| 11 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 858.` |
| 12 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {IMPORT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 984.` |
| 13 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.reserved = const<br>	∧ +1.roles not in {IMPORT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 172.` |
| 14 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.reserved not in {const}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.985. Support: 2614.` |
| 15 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 977.` |
| 16 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.934. Support: 854.` |
| 17 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {ARGUMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 401.` |
| 18 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 412.` |
| 19 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -4.diff_line = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 332.` |
| 20 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved = (<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 163.` |
| 21 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -1.length ≥ 4<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>	∧ ^2.roles not in {VISIBILITY}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 600.` |
| 22 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>	∧ ^2.roles not in {VISIBILITY}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 17101.` |
| 23 | `  •••start_col ≤ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.length = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 503.` |
| 24 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 1.000. Support: 4678.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 6309.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 1388.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.930. Support: 3649.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 648.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 4477.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 1739.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 160.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 952.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≥ 3<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 621.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≤ 2<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {ARGUMENT, EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 588.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≤ 2<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION, RIGHT} and not in {ARGUMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 254.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.length ≤ 2<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION} and not in {ARGUMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 203.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 888.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 777.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, if}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 361.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, if}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 339.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 176.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -3.length ≥ 4<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.994. Support: 432.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -2.reserved = =<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 353.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 177.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -2.reserved not in {=}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 38463.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length = 0<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.987. Support: 422.` |
| 47 | `  -1.reserved not in {)}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 16591.` |
| 48 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.964. Support: 545.` |
| 49 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 6427.` |
| 50 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 1.000. Support: 3119.` |
| 51 | `  •••start_col ≥ 13<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {FUNCTION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.960. Support: 188.` |
| 52 | `  •••start_col ≥ 13<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {FUNCTION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 1073.` |
| 53 | `  •••start_col ≥ 13<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -3.roles not in {PATHNAME}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {STRING} and not in {FUNCTION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.978. Support: 580.` |
| 54 | `  •••start_col ≤ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.955. Support: 586.` |
| 55 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved = ,<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 177.` |
| 56 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.roles in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.995. Support: 762.` |
| 57 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 837.` |
| 58 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = }<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 906.` |
| 59 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {,, }}<br>	∧ +3.roles not in {VALUE}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.966. Support: 3110.` |
| 60 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 989.` |
| 61 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.934. Support: 854.` |
| 62 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {ARGUMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 417.` |
| 63 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = [<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.921. Support: 210.` |
| 64 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.label in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 401.` |
| 65 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 350.` |
| 66 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 21<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_col ≥ 3<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 18061.` |
| 67 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 21<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_col ≤ 2<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 208.` |
| 68 | `  •••start_col ≤ 5<br>	∧ -1.diff_col = 0<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 418.` |
| 69 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 1.000. Support: 4752.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 6299.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.920. Support: 3521.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 653.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 4564.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 1765.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 170.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 972.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {ARGUMENT, EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 586.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≥ 2<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION} and not in {ARGUMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 736.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 917.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 764.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -3.length ≥ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 471.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -2.reserved = =<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 336.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 177.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -2.reserved not in {=}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 38395.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length = 0<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 431.` |
| 86 | `  -1.reserved not in {)}<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 16466.` |
| 87 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.966. Support: 578.` |
| 88 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 6459.` |
| 89 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 1.000. Support: 2926.` |
| 90 | `  •••start_col ≥ 13<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {FUNCTION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.958. Support: 179.` |
| 91 | `  •••start_col ≥ 13<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {FUNCTION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 1004.` |
| 92 | `  •••start_col ≥ 13<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -3.roles not in {PATHNAME}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {LITERAL} and not in {FUNCTION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.964. Support: 627.` |
| 93 | `  •••start_col ≤ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.943. Support: 566.` |
| 94 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved = ,<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 174.` |
| 95 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {,}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.938. Support: 3584.` |
| 96 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.roles in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.996. Support: 706.` |
| 97 | `  •••start_col ≥ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {VALUE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.934. Support: 3560.` |
| 98 | `  •••start_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 1001.` |
| 99 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.941. Support: 827.` |
| 100 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.length ≥ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 371.` |
| 101 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.length ≤ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 340.` |
| 102 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved = (<br>	∧ -3.diff_col ≥ 5<br>	∧ -3.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 132.` |
| 103 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 16877.` |
| 104 | `  •••start_col ≤ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 480.` |
| 105 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 1.000. Support: 4797.` |
| 106 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 6176.` |
| 107 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 1387.` |
| 108 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.928. Support: 3530.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 684.` |
| 110 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 4497.` |
| 111 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 1713.` |
| 112 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 198.` |
| 113 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 1131.` |
| 114 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {(, )}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 631.` |
| 115 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {(, )}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 329.` |
| 116 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 853.` |
| 117 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 736.` |
| 118 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -3.length ≥ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.980. Support: 515.` |
| 119 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -2.reserved = =<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 353.` |
| 120 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 206.` |
| 121 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length = 0<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.983. Support: 387.` |
| 122 | `  -1.reserved not in {)}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.991. Support: 16565.` |
| 123 | `  •••start_col ≥ 13<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {FUNCTION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⏎<br>Confidence: 0.970. Support: 186.` |
| 124 | `  •••start_col ≥ 13<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {FUNCTION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 1116.` |
| 125 | `  •••start_col ≥ 13<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {import}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {STRING} and not in {FUNCTION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 577.` |
| 126 | `  •••start_col ≤ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⏎<br>Confidence: 0.935. Support: 580.` |
| 127 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.roles in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.993. Support: 789.` |
| 128 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 936.` |
| 129 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {,, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.932. Support: 3532.` |
| 130 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 969.` |
| 131 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.931. Support: 820.` |
| 132 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {ARGUMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 388.` |
| 133 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.length ≥ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 400.` |
| 134 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.length ≤ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 312.` |
| 135 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 17016.` |
| 136 | `  •••start_col ≤ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_col = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 457.` |
| 137 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 912.` |
| 138 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≥ 3<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 635.` |
| 139 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≤ 2<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {ARGUMENT, EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 626.` |
| 140 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≤ 2<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION, RIGHT} and not in {ARGUMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 236.` |
| 141 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -3.length ≥ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.975. Support: 454.` |
| 142 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 187.` |
| 143 | `  •••start_col ≥ 27<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -2.reserved not in {=}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = TryStatement<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 153.` |
| 144 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -2.label in {<-space>}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 961.` |
| 145 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved = (<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 149.` |
| 146 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -5.diff_line = 0<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 7712.` |
| 147 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 16921.` |
| 148 | `  •••start_col ≥ 19<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 567.` |
| 149 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 10117.` |
| 150 | `  -1.reserved = )<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved not in {}}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 165.` |
| 151 | `  •••start_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {FUNCTION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.954. Support: 207.` |
| 152 | `  •••start_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {FUNCTION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 1079.` |
| 153 | `  •••start_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {import}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {STRING} and not in {FUNCTION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.958. Support: 608.` |
| 154 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 914.` |
| 155 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {,, }}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.931. Support: 3633.` |
| 156 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.938. Support: 802.` |
| 157 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = [<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {STRING}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>⇒ y = '<br>Confidence: 0.930. Support: 193.` |
| 158 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.length ≥ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 392.` |
| 159 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.length ≤ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 288.` |
| 160 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -5.diff_line = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 1721.` |
| 161 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 15554.` |
| 162 | `  •••start_col ≤ 4<br>	∧ •••start_line ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 462.` |
| 163 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≥ 2<br>	∧ -2.reserved not in {)}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 1097.` |
| 164 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≤ 1<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 560.` |
| 165 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≤ 1<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 807.` |
| 166 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.length ≤ 1<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 175.` |
| 167 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.986. Support: 521.` |
| 168 | `  -1.reserved not in {)}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 16637.` |
| 169 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.972. Support: 558.` |
| 170 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 6445.` |
| 171 | `  •••start_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.982. Support: 1066.` |
| 172 | `  •••start_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -3.roles in {IMPORT}<br>	∧ +1.reserved not in {import}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.930. Support: 337.` |
| 173 | `  •••start_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -3.roles not in {IMPORT}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.956. Support: 598.` |
| 174 | `  -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 747.` |
| 175 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 870.` |
| 176 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {IMPORT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 1000.` |
| 177 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -5.label in {<space>}<br>	∧ +1.roles not in {IMPORT, VALUE}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {VALUE}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.967. Support: 135.` |
| 178 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {IMPORT, VALUE}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {VALUE}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.961. Support: 3140.` |
| 179 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.label in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 380.` |
| 180 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 312.` |
| 181 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -5.diff_line = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 1745.` |
| 182 | `  •••start_col ≥ 5<br>	∧ -1.diff_col ≥ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 644.` |
| 183 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 15378.` |
| 184 | `  •••start_col ≤ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≤ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 458.` |
| 185 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {)}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 1074.` |
| 186 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 831.` |
| 187 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 638.` |
| 188 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 185.` |
| 189 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -3.length ≥ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 464.` |
| 190 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -2.label in {<-space>}<br>	∧ -4.diff_line ≤ 1<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {const}<br>	∧ +5.roles not in {IMPORT}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 904.` |
| 191 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -4.diff_line ≤ 1<br>	∧ -5.diff_line = 0<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {const}<br>	∧ +5.roles not in {IMPORT}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 7906.` |
| 192 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -2.reserved not in {=}<br>	∧ -4.diff_line ≤ 1<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {const}<br>	∧ +5.roles not in {IMPORT}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 27644.` |
| 193 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length = 0<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.988. Support: 448.` |
| 194 | `  -1.reserved not in {)}<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 16524.` |
| 195 | `  •••start_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {FUNCTION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.937. Support: 197.` |
| 196 | `  •••start_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {FUNCTION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.979. Support: 1075.` |
| 197 | `  •••start_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -3.roles not in {IMPORT}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {STRING} and not in {FUNCTION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 565.` |
| 198 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ -4.diff_offset ≥ 12<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {,}<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 712.` |
| 199 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {,}<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.967. Support: 2818.` |
| 200 | `  -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 734.` |
| 201 | `  •••start_col ≥ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {VALUE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.939. Support: 3481.` |
| 202 | `  •••start_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 989.` |
| 203 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.937. Support: 865.` |
| 204 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {ARGUMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 422.` |
| 205 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.label in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 406.` |
| 206 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 344.` |
| 207 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_col ≥ 3<br>	∧ -3.diff_offset ≥ 5<br>	∧ -5.diff_line = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 1834.` |
| 208 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_col ≥ 3<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 15742.` |
| 209 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_col ≤ 2<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 208.` |
| 210 | `  •••start_col ≤ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.length = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 481.` |
| 211 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 1038.` |
| 212 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≥ 2<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 831.` |
| 213 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≤ 1<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {ARGUMENT, EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 640.` |
| 214 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -3.length ≥ 3<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.990. Support: 461.` |
| 215 | `  •••start_col ≥ 13<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -3.roles not in {PATHNAME}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {STRING} and not in {FUNCTION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.965. Support: 559.` |
| 216 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {,}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>	∧ ^2.roles in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.972. Support: 2135.` |
| 217 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {ARGUMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 427.` |
| 218 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 441.` |
| 219 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -5.diff_line = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 309.` |
| 220 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_col ≥ 3<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 17520.` |
| 221 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_col ≤ 2<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 206.` |
| 222 | `  •••start_col ≤ 14<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.diff_offset ≤ 5<br>	∧ -5.label in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 162.` |
| 223 | `  •••start_col ≤ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.diff_offset ≤ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 435.` |
| 224 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length = 0<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.988. Support: 388.` |
| 225 | `  +1.reserved not in {}}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 16867.` |
| 226 | `  •••start_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -3.roles not in {IMPORT}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {LITERAL} and not in {FUNCTION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 573.` |
| 227 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.995. Support: 754.` |
| 228 | `  •••start_col ≥ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {MAP} and not in {VALUE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>	∧ ^2.roles in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.945. Support: 467.` |
| 229 | `  •••start_col ≥ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {MAP, VALUE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.987. Support: 2515.` |
| 230 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.933. Support: 808.` |
| 231 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {FUNCTION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 417.` |
| 232 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {FUNCTION}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 318.` |
| 233 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {FUNCTION}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 411.` |
| 234 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {FUNCTION, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved = (<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {OPERATOR} and not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 166.` |
| 235 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {FUNCTION, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 18106.` |
| 236 | `  •••start_col ≤ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_offset ≤ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 454.` |
| 237 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION} and not in {ARGUMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 746.` |
| 238 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -3.length ≥ 4<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 470.` |
| 239 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 972.` |
| 240 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -2.reserved = (<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 152.` |
| 241 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -5.diff_line = 0<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 7983.` |
| 242 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -2.reserved not in {=}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 28244.` |
| 243 | `  -1.roles not in {LITERAL}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 16997.` |
| 244 | `  •••start_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -3.roles not in {IMPORT}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles in {LITERAL} and not in {FUNCTION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.941. Support: 673.` |
| 245 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = ,<br>	∧ +5.reserved = from<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 141.` |
| 246 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {ARGUMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 458.` |
| 247 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.label in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 394.` |
| 248 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 326.` |
| 249 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved = (<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 164.` |
| 250 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -1.roles not in {ARGUMENT, LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 18015.` |
| 251 | `  •••start_col ≤ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -4.length = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 458.` |
| 252 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {(, )}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 348.` |
| 253 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 906.` |
| 254 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {(, )}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 655.` |
| 255 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {(, )}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≥ 4<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 505.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.458823529411765, "max_conf": 0.9998957514762878, "max_support": 38463, "min_conf": 0.920051097869873, "min_support": 132, "num_rules": 255}}
```
</details>
