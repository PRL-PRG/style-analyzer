# Model report for file:///tmp/top-repos-quality-repos-_eamxpwh/cultivate-py.git HEAD 9c6850e3bc54ba8192c394cc03b017a95aff103a

### Dump

```json
{'created_at': '2021-08-22 00:45:54',
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
 'size': '30.9 kB',
 'tags': [],
 'uuid': '2f145c1b-f6ad-444f-b916-6a553a5f70d3',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-_eamxpwh/cultivate-py.git 9c6850e3bc54ba8192c394cc03b017a95aff103a

# javascript
122 rules, avg.len. 13.6
## train
PPCR: 0.947759
### report
macro
{'f1-score': 0.4175647644705072,
 'precision': 0.4876993682845869,
 'recall': 0.4010776346640589,
 'support': 434068}
micro
{'f1-score': 0.9716841600855165,
 'precision': 0.9716841600855165,
 'recall': 0.9716841600855165,
 'support': 434068}
weighted
{'f1-score': 0.9678643993780008,
 'precision': 0.966549703619904,
 'recall': 0.9716841600855165,
 'support': 434068}
### report_full
macro
{'f1-score': 0.3720324081830407,
 'precision': 0.4876993682845869,
 'recall': 0.340000620709033,
 'support': 457994}
micro
{'f1-score': 0.9456226136748342,
 'precision': 0.9716841600855165,
 'recall': 0.9209225448368319,
 'support': 457994}
weighted
{'f1-score': 0.9354994133019883,
 'precision': 0.9599410992021271,
 'recall': 0.9209225448368319,
 'support': 457994}
## test
PPCR: 0.952752
### report
macro
{'f1-score': 0.4103932266500344,
 'precision': 0.4877725222431535,
 'recall': 0.39293550150546586,
 'support': 100663}
micro
{'f1-score': 0.9713797522426314,
 'precision': 0.9713797522426314,
 'recall': 0.9713797522426314,
 'support': 100663}
weighted
{'f1-score': 0.967231149556393,
 'precision': 0.9656259162087553,
 'recall': 0.9713797522426314,
 'support': 100663}
### report_full
macro
{'f1-score': 0.36965845629735805,
 'precision': 0.4877725222431535,
 'recall': 0.3393026697336739,
 'support': 105655}
micro
{'f1-score': 0.9478765788733896,
 'precision': 0.9713797522426314,
 'recall': 0.9254838862334959,
 'support': 105655}
weighted
{'f1-score': 0.9381428283549447,
 'precision': 0.960781251120808,
 'recall': 0.9254838862334959,
 'support': 105655}
```

## javascript
### Summary
83 rules, avg.len. 12.3

| | |
|-|-|
|Min support|90|
|Max support|51812|
|Min confidence|0.9206896424293518|
|Max confidence|0.9999507665634155|

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
| 1 | `  -1.roles in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.974. Support: 749.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.975. Support: 139.` |
| 3 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.996. Support: 129.` |
| 4 | `  -1.reserved not in {)}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.999. Support: 9794.` |
| 5 | `  -1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2510.` |
| 6 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.929. Support: 1607.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.979. Support: 214.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 1622.` |
| 9 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.996. Support: 127.` |
| 10 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 1166.` |
| 11 | `  -1.label not in {<space>}<br>	∧ -1.reserved = function<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 888.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.995. Support: 99.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {function, {}<br>	∧ +1.reserved = (<br>	∧ +2.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 234.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -2.roles in {BINARY}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.length ≤ 20<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 548.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -2.roles in {BINARY}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.length ≤ 20<br>	∧ +2.reserved = (<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 128.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -2.roles not in {BINARY}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.length ≤ 20<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 34595.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {function, {}<br>	∧ -2.internal_type = Identifier<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 141.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.length ≤ 1<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.internal_type = LogicalExpression<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 604.` |
| 19 | `  •••start_col ≥ 26<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved = ;<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 128.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved not in {;}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 4610.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved not in {;}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 772.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = )<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved not in {;}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {), ]}<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 199.` |
| 23 | `  •••start_line ≤ 238<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ), [, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved not in {;}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 91.` |
| 24 | `  -1.roles in {EXPRESSION, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.967. Support: 7122.` |
| 25 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 6683.` |
| 26 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +2.reserved not in {:}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 1194.` |
| 27 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +2.reserved not in {:}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 92.` |
| 28 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.929. Support: 823.` |
| 29 | `  -1.roles in {EXPRESSION, INCOMPLETE} and not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.997. Support: 154.` |
| 30 | `  -1.roles in {EXPRESSION} and not in {INCOMPLETE, STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 288.` |
| 31 | `  -1.roles in {EXPRESSION} and not in {INCOMPLETE, STRING}<br>	∧ -2.diff_offset ≤ 21<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ;<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 4418.` |
| 32 | `  -1.roles in {EXPRESSION} and not in {INCOMPLETE, STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 51812.` |
| 33 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≤ 5<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.946. Support: 1729.` |
| 34 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 18504.` |
| 35 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.roles in {IDENTIFIER}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {AssignmentExpression}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.924. Support: 5544.` |
| 36 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 10152.` |
| 37 | `  •••start_col ≤ 33<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {return}<br>	∧ -5.diff_offset ≤ 19<br>	∧ +1.reserved = }<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {RIGHT}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.966. Support: 218.` |
| 38 | `  •••start_col ≤ 13<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 312.` |
| 39 | `  •••start_col ≤ 13<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 1<br>	∧ -2.label in {<-tab>}<br>	∧ +1.reserved = }<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.994. Support: 237.` |
| 40 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 7101.` |
| 41 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.988. Support: 4170.` |
| 42 | `  -1.label not in {<space>}<br>	∧ -1.reserved = [<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.985. Support: 167.` |
| 43 | `  -1.label not in {<space>}<br>	∧ -1.reserved = -<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 166.` |
| 44 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, -, [, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 6942.` |
| 45 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -3.roles in {LITERAL}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = ObjectExpression<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 90.` |
| 46 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -3.roles not in {LITERAL}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 5246.` |
| 47 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {STRING} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = '<br>Confidence: 0.960. Support: 137.` |
| 48 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT, STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 194.` |
| 49 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {CALL}<br>	∧ -4.reserved not in {.}<br>	∧ -5.diff_line = 0<br>	∧ -5.diff_offset ≥ 32<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 276.` |
| 50 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≥ 5<br>	∧ -4.reserved not in {.}<br>	∧ -5.diff_line = 0<br>	∧ -5.diff_offset ≤ 31<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 4666.` |
| 51 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 4<br>	∧ -4.reserved not in {.}<br>	∧ -5.diff_line = 0<br>	∧ -5.diff_offset ≤ 31<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 150.` |
| 52 | `  -1.diff_col ≥ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>} and not in {<space>}<br>	∧ -2.length ≥ 3<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 2834.` |
| 53 | `  -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>} and not in {<space>}<br>	∧ -2.length ≥ 3<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 815.` |
| 54 | `  •••start_line ≥ 201<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>} and not in {<space>}<br>	∧ -2.length ≤ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 1468.` |
| 55 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 41<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -4.diff_line ≥ 3<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 1169.` |
| 56 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 1218.` |
| 57 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -4.diff_offset ≤ 6<br>	∧ -5.diff_offset ≥ 28<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles in {LIST} and not in {OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 109.` |
| 58 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.reserved = (<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {LIST, OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 554.` |
| 59 | `  -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = '<br>Confidence: 0.987. Support: 414.` |
| 60 | `  -1.diff_col ≥ 2<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {THIS}<br>	∧ -4.reserved not in {,}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles in {FUNCTION} and not in {LIST, OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 108.` |
| 61 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {THIS}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {FUNCTION, LIST, OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 248.` |
| 62 | `  -1.diff_offset ≤ 123<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {ARGUMENT, THIS}<br>	∧ -3.diff_offset ≥ 16<br>	∧ -3.length ≥ 2<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression, IfStatement, MemberExpression}<br>	∧ ^1.roles not in {FUNCTION, LIST, OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.965. Support: 5979.` |
| 63 | `  -1.diff_offset ≤ 123<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {ARGUMENT, THIS}<br>	∧ -3.diff_offset ≤ 15<br>	∧ -3.roles in {VALUE}<br>	∧ -3.length ≥ 2<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression, IfStatement, MemberExpression}<br>	∧ ^1.roles not in {FUNCTION, LIST, OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.985. Support: 711.` |
| 64 | `  -1.diff_offset ≤ 123<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {ARGUMENT, THIS}<br>	∧ -3.label not in {<newline>}<br>	∧ -3.length ≤ 1<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression, IfStatement, MemberExpression}<br>	∧ ^1.roles in {MAP} and not in {DECLARATION, FILE, FUNCTION, LIST, OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.970. Support: 1912.` |
| 65 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 4030.` |
| 66 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 4062.` |
| 67 | `  -1.diff_col ≥ 9<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.993. Support: 230.` |
| 68 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 2422.` |
| 69 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.972. Support: 338.` |
| 70 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = ]<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 194.` |
| 71 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {]}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles in {LITERAL} and not in {ASSIGNMENT, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 435.` |
| 72 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {LIST, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.934. Support: 99.` |
| 73 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = [<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.979. Support: 118.` |
| 74 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = [<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 127.` |
| 75 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 229.` |
| 76 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = -<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 157.` |
| 77 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 474.` |
| 78 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, -, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≥ 5<br>	∧ -4.diff_line = 0<br>	∧ +1.reserved not in {(, ), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>	∧ ^2.internal_type = BinaryExpression<br>⇒ y = ␣<br>Confidence: 0.996. Support: 112.` |
| 79 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, -, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_line = 0<br>	∧ +1.reserved not in {(, ), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 1491.` |
| 80 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ +3.roles in {LEFT}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 121.` |
| 81 | `  -1.diff_col ≤ 6<br>	∧ -1.label in {'} and not in {<space>}<br>	∧ -1.reserved not in {(, -, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ +3.roles not in {LEFT}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 330.` |
| 82 | `  -1.diff_col ≤ 6<br>	∧ -1.diff_offset ≥ 6<br>	∧ -1.label not in {', <space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≥ 5<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ +3.roles not in {LEFT}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 193.` |
| 83 | `  -1.diff_col ≤ 6<br>	∧ -1.diff_offset ≤ 5<br>	∧ -1.label not in {', <space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ +3.roles not in {LEFT}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 19282.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 12.337349397590362, "max_conf": 0.9999507665634155, "max_support": 51812, "min_conf": 0.9206896424293518, "min_support": 90, "num_rules": 83}}
```
</details>
