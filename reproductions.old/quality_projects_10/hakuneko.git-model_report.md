# Model report for file:///tmp/top-repos-quality-repos-lpx_nbtf/hakuneko.git HEAD 55785df0286038654e61cc66d3f30792625e172a

### Dump

```json
{'created_at': '2021-08-24 21:25:29',
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
 'size': '29.3 kB',
 'tags': [],
 'uuid': '669218bc-5da6-4437-9583-5c54f5ea1c00',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-lpx_nbtf/hakuneko.git 55785df0286038654e61cc66d3f30792625e172a

# javascript
430 rules, avg.len. 9.6
## train
PPCR: 0.984636
### report
macro
{'f1-score': 0.6124245866529286,
 'precision': 0.619755190530376,
 'recall': 0.6120188119956607,
 'support': 215401}
micro
{'f1-score': 0.9523446966355774,
 'precision': 0.9523446966355774,
 'recall': 0.9523446966355774,
 'support': 215401}
weighted
{'f1-score': 0.9488536184229431,
 'precision': 0.9466748128497362,
 'recall': 0.9523446966355774,
 'support': 215401}
### report_full
macro
{'f1-score': 0.6049750629693452,
 'precision': 0.619755190530376,
 'recall': 0.5981309828358468,
 'support': 218762}
micro
{'f1-score': 0.9449722799962227,
 'precision': 0.9523446966355774,
 'recall': 0.937713131165376,
 'support': 218762}
weighted
{'f1-score': 0.9409430159988317,
 'precision': 0.945692761146652,
 'recall': 0.937713131165376,
 'support': 218762}
## test
PPCR: 0.984948
### report
macro
{'f1-score': 0.6067123728244707,
 'precision': 0.6146176526139685,
 'recall': 0.608561036598224,
 'support': 55554}
micro
{'f1-score': 0.9519566547863341,
 'precision': 0.9519566547863341,
 'recall': 0.9519566547863341,
 'support': 55554}
weighted
{'f1-score': 0.947528292832157,
 'precision': 0.9449579406365503,
 'recall': 0.9519566547863341,
 'support': 55554}
### report_full
macro
{'f1-score': 0.5985821656194664,
 'precision': 0.6146176526139685,
 'recall': 0.5933380600514053,
 'support': 56403}
micro
{'f1-score': 0.944737711800066,
 'precision': 0.9519566547863341,
 'recall': 0.9376274311650089,
 'support': 56403}
weighted
{'f1-score': 0.9395821495704796,
 'precision': 0.9435842428819196,
 'recall': 0.9376274311650089,
 'support': 56403}
```

## javascript
### Summary
236 rules, avg.len. 9.6

| | |
|-|-|
|Min support|142|
|Max support|32863|
|Min confidence|0.9203233122825623|
|Max confidence|0.9997075796127319|

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
| 1 | `  -1.roles in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.993. Support: 341.` |
| 2 | `  +1.reserved not in {)}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.986. Support: 243.` |
| 3 | `  -1.reserved = )<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {STRING}<br>	∧ +4.roles in {FUNCTION}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.932. Support: 155.` |
| 4 | `  -1.reserved not in {)}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {STRING}<br>	∧ +4.roles in {FUNCTION}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 513.` |
| 5 | `  +1.reserved not in {), }}<br>	∧ +1.roles not in {STRING}<br>	∧ +4.roles not in {FUNCTION}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 32538.` |
| 6 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.995. Support: 8223.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 6205.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved = class<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.999. Support: 463.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {class}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 6738.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.roles in {TYPE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.975. Support: 608.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -3.diff_col ≤ 30<br>	∧ -3.length ≤ 1<br>	∧ -4.diff_offset ≥ 19<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 412.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1710.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≥ 16<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.928. Support: 1361.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≤ 15<br>	∧ -4.diff_offset ≥ 14<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 733.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 2<br>	∧ +2.length ≥ 3<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 974.` |
| 16 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.922. Support: 1211.` |
| 17 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 262.` |
| 18 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 679.` |
| 19 | `  •••start_col ≥ 7<br>	∧ •••start_line ≤ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 961.` |
| 20 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 190.` |
| 21 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {;}<br>	∧ +1.length ≤ 68<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.950. Support: 548.` |
| 22 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ +1.length ≤ 68<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 25806.` |
| 23 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.941. Support: 769.` |
| 24 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 719.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 8045.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 5234.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 230.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 205.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 1788.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 548.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.993. Support: 347.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 706.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≥ 13<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 195.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 12<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 6773.` |
| 35 | `  •••start_col ≥ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 28281.` |
| 36 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 872.` |
| 37 | `  -1.roles in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.990. Support: 339.` |
| 38 | `  +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 267.` |
| 39 | `  -1.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +4.roles in {FUNCTION}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 498.` |
| 40 | `  +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +4.roles not in {FUNCTION}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 32534.` |
| 41 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.996. Support: 8048.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 6202.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved = class<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.999. Support: 483.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {class}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.949. Support: 6868.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.roles in {TYPE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.962. Support: 643.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -3.diff_col ≤ 30<br>	∧ -3.diff_offset ≥ 8<br>	∧ -4.diff_col ≤ 3<br>	∧ -4.diff_offset ≥ 19<br>	∧ -4.reserved not in {}}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 212.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -3.diff_col ≤ 30<br>	∧ -3.diff_offset ≤ 7<br>	∧ -4.diff_offset ≥ 19<br>	∧ -4.reserved not in {}}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 349.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1677.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≥ 16<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.922. Support: 1337.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≤ 15<br>	∧ -4.diff_offset ≥ 14<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 787.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 2<br>	∧ +2.length ≥ 3<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 946.` |
| 52 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.922. Support: 1230.` |
| 53 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 296.` |
| 54 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 700.` |
| 55 | `  •••start_col ≥ 7<br>	∧ •••start_line ≤ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 988.` |
| 56 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {BLOCK}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 213.` |
| 57 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.roles not in {BLOCK}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 193.` |
| 58 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 7<br>	∧ +1.length ≤ 64<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 546.` |
| 59 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ +1.length ≤ 64<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 26015.` |
| 60 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ -2.length ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.941. Support: 822.` |
| 61 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.length = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 745.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 8082.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 5065.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 221.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {FUNCTION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 185.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION} and not in {FUNCTION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 1791.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.991. Support: 289.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 760.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved = ,<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 677.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -2.length ≥ 13<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 224.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 12<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 6771.` |
| 72 | `  •••start_col ≥ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 26933.` |
| 73 | `  •••start_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 1546.` |
| 74 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.993. Support: 358.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +4.roles in {TYPE}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.999. Support: 446.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +4.roles not in {TYPE}<br>	∧ ^1.roles in {BLOCK} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 6682.` |
| 77 | `  •••start_line ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +4.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎⏎<br>Confidence: 0.931. Support: 152.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.roles not in {TYPE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FUNCTION} and not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.950. Support: 2829.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -3.diff_col ≤ 30<br>	∧ -3.diff_offset ≤ 7<br>	∧ -4.diff_offset ≥ 19<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 346.` |
| 80 | `  •••start_col ≤ 71<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -5.diff_line = 0<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {INCOMPLETE} and not in {CALLEE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 692.` |
| 81 | `  •••start_col ≤ 71<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -5.diff_line = 0<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {CALLEE, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 1888.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1685.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≥ 16<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.924. Support: 1303.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≥ 3<br>	∧ -2.diff_offset ≤ 15<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 705.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 2<br>	∧ +2.length ≥ 3<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 953.` |
| 86 | `  •••start_col ≥ 7<br>	∧ •••start_line ≤ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 929.` |
| 87 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 261.` |
| 88 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 700.` |
| 89 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 336.` |
| 90 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 181.` |
| 91 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.length ≤ 16<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 7<br>	∧ +1.length ≤ 64<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 590.` |
| 92 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.length ≤ 16<br>	∧ -2.reserved not in {(}<br>	∧ +1.length ≤ 64<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 25856.` |
| 93 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ -3.length ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.935. Support: 823.` |
| 94 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.length = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 704.` |
| 95 | `  •••start_line ≤ 128<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -5.label not in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.920. Support: 3702.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {FUNCTION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 166.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION} and not in {FUNCTION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 1791.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {BINARY}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION, FUNCTION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 377.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {'}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 762.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -2.length ≥ 13<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 196.` |
| 101 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved = .<br>	∧ -2.length ≤ 12<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 722.` |
| 102 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {.}<br>	∧ -2.length ≤ 12<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 4945.` |
| 103 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.938. Support: 153.` |
| 104 | `  •••start_col ≥ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved not in {(, ,}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 28183.` |
| 105 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved not in {(, ,}<br>	∧ +1.reserved not in {(, ), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 932.` |
| 106 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {class}<br>	∧ ^1.roles in {BLOCK} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 6904.` |
| 107 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≤ 15<br>	∧ -4.diff_offset ≥ 14<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 776.` |
| 108 | `  •••start_col ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≥ 3<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 2<br>	∧ +2.roles in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 158.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 2<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 1024.` |
| 110 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 291.` |
| 111 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 714.` |
| 112 | `  •••start_col ≥ 7<br>	∧ •••start_line ≤ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 982.` |
| 113 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.997. Support: 175.` |
| 114 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.roles in {BLOCK}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 175.` |
| 115 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ +1.length ≤ 64<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 25112.` |
| 116 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles not in {BLOCK}<br>	∧ +1.length ≤ 2<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 1302.` |
| 117 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.936. Support: 738.` |
| 118 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 719.` |
| 119 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<-space>, <space>}<br>	∧ -2.length ≥ 13<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 209.` |
| 120 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.993. Support: 361.` |
| 121 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.length ≤ 1<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 523.` |
| 122 | `  +1.reserved not in {)}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 255.` |
| 123 | `  +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.930. Support: 207.` |
| 124 | `  -1.reserved = )<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {STRING}<br>	∧ +4.roles in {FUNCTION}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.926. Support: 183.` |
| 125 | `  -1.reserved not in {)}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {STRING}<br>	∧ +4.roles in {FUNCTION}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 499.` |
| 126 | `  +1.reserved not in {), }}<br>	∧ +1.roles not in {STRING}<br>	∧ +4.roles not in {FUNCTION}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 32446.` |
| 127 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {class}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 6954.` |
| 128 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -3.diff_col ≤ 30<br>	∧ -3.diff_offset ≥ 8<br>	∧ -4.diff_col ≤ 2<br>	∧ -4.diff_offset ≥ 19<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 179.` |
| 129 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -3.diff_col ≤ 30<br>	∧ -3.diff_offset ≤ 7<br>	∧ -4.diff_offset ≥ 19<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 381.` |
| 130 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≤ 15<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 779.` |
| 131 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -2.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 255.` |
| 132 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -2.reserved not in {,}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 699.` |
| 133 | `  •••start_col ≥ 7<br>	∧ •••start_line ≤ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 1010.` |
| 134 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 177.` |
| 135 | `  •••start_col ≤ 30<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles not in {BLOCK}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 546.` |
| 136 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles not in {BLOCK}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 23404.` |
| 137 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ -3.length ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.928. Support: 839.` |
| 138 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.length = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 687.` |
| 139 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 210.` |
| 140 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 1822.` |
| 141 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 573.` |
| 142 | `  •••start_col ≥ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 27944.` |
| 143 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 905.` |
| 144 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +4.roles not in {TYPE}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.948. Support: 6827.` |
| 145 | `  •••start_line ≤ 120<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ -3.roles not in {TYPE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.922. Support: 3375.` |
| 146 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -3.diff_col ≤ 23<br>	∧ -3.length ≤ 1<br>	∧ -4.diff_offset ≥ 19<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 388.` |
| 147 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -5.diff_line = 0<br>	∧ -5.label not in {<+space>}<br>	∧ -5.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 173.` |
| 148 | `  •••start_line ≥ 52<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -5.diff_line = 0<br>	∧ -5.label not in {<+space>}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 3<br>	∧ ^1.roles in {INITIALIZATION} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 142.` |
| 149 | `  •••start_line ≤ 51<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -5.diff_line = 0<br>	∧ -5.label not in {<+space>}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 1572.` |
| 150 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -2.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 250.` |
| 151 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -2.reserved not in {,}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 713.` |
| 152 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {BLOCK} and not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 358.` |
| 153 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 200.` |
| 154 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.length ≤ 16<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 7<br>	∧ +1.length ≤ 68<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 588.` |
| 155 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.length ≤ 16<br>	∧ -2.reserved not in {(}<br>	∧ +1.length ≤ 68<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 25546.` |
| 156 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -2.length ≥ 13<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 215.` |
| 157 | `  •••start_col ≥ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 28244.` |
| 158 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 926.` |
| 159 | `  -1.roles in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.999. Support: 341.` |
| 160 | `  +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.990. Support: 257.` |
| 161 | `  -1.reserved = )<br>	∧ -5.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +4.roles in {FUNCTION}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ⏎<br>Confidence: 0.969. Support: 145.` |
| 162 | `  -1.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +4.roles in {FUNCTION}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.977. Support: 491.` |
| 163 | `  +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +4.roles not in {FUNCTION}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.996. Support: 32863.` |
| 164 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.995. Support: 8040.` |
| 165 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 6017.` |
| 166 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved = class<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⏎<br>Confidence: 0.999. Support: 471.` |
| 167 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {class}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {SCOPE}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 6636.` |
| 168 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.roles in {TYPE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.958. Support: 588.` |
| 169 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -3.diff_col ≤ 23<br>	∧ -4.diff_offset ≥ 19<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 301.` |
| 170 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1681.` |
| 171 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≥ 16<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ⏎<br>Confidence: 0.933. Support: 1327.` |
| 172 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≤ 15<br>	∧ -4.diff_offset ≥ 14<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 736.` |
| 173 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 2<br>	∧ +2.length ≥ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 981.` |
| 174 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {FILE}<br>⇒ y = '<br>Confidence: 0.998. Support: 262.` |
| 175 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 761.` |
| 176 | `  •••start_col ≥ 7<br>	∧ •••start_line ≤ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {FILE}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 969.` |
| 177 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LITERAL, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 184.` |
| 178 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ +1.length ≤ 66<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 25153.` |
| 179 | `  •••start_col ≤ 6<br>	∧ •••start_line ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = ⏎⏎<br>Confidence: 0.932. Support: 747.` |
| 180 | `  •••start_col ≤ 6<br>	∧ •••start_line ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 726.` |
| 181 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 8157.` |
| 182 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 5311.` |
| 183 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {FUNCTION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 161.` |
| 184 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION} and not in {FUNCTION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 1752.` |
| 185 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION, FUNCTION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 415.` |
| 186 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 782.` |
| 187 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.983. Support: 326.` |
| 188 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved = ,<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 658.` |
| 189 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -2.length ≥ 13<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 215.` |
| 190 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 12<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 6615.` |
| 191 | `  •••start_col ≥ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 26963.` |
| 192 | `  •••start_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 1539.` |
| 193 | `  -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ -5.internal_type not in {Identifier}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 322.` |
| 194 | `  -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 13<br>	∧ -2.label not in {<space>}<br>	∧ -5.internal_type not in {Identifier}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 274.` |
| 195 | `  +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.995. Support: 279.` |
| 196 | `  -1.reserved = )<br>	∧ -4.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +4.roles in {FUNCTION}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.935. Support: 146.` |
| 197 | `  -1.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +4.roles in {FUNCTION}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 505.` |
| 198 | `  +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +4.roles not in {FUNCTION}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 32777.` |
| 199 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = export<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.995. Support: 457.` |
| 200 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {export}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 6788.` |
| 201 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≤ 15<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 716.` |
| 202 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 176.` |
| 203 | `  •••start_col ≤ 29<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles not in {BLOCK}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.972. Support: 527.` |
| 204 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles not in {BLOCK}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 23426.` |
| 205 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ -2.length ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.933. Support: 818.` |
| 206 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.length = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 705.` |
| 207 | `  -1.reserved = )<br>	∧ -4.label in {<newline>}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {STRING}<br>	∧ +4.roles in {FUNCTION}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 160.` |
| 208 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {export}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {BLOCK} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 6889.` |
| 209 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -3.diff_col ≤ 29<br>	∧ -3.diff_offset ≥ 8<br>	∧ -4.diff_col ≤ 2<br>	∧ -4.diff_offset ≥ 19<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 161.` |
| 210 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -3.diff_col ≤ 29<br>	∧ -3.diff_offset ≤ 7<br>	∧ -4.diff_offset ≥ 19<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 386.` |
| 211 | `  •••start_col ≥ 41<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -5.diff_line = 0<br>	∧ -5.label not in {<+space>}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {INCOMPLETE} and not in {CALLEE, DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 335.` |
| 212 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -2.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 287.` |
| 213 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -2.reserved not in {,}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 686.` |
| 214 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles not in {BLOCK}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 24162.` |
| 215 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.roles in {EXPRESSION} and not in {BLOCK}<br>	∧ +1.length ≤ 15<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 2259.` |
| 216 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 1677.` |
| 217 | `  •••start_col ≥ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 27452.` |
| 218 | `  •••start_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.length ≤ 1<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 1426.` |
| 219 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.987. Support: 344.` |
| 220 | `  •••start_col ≤ 29<br>	∧ -1.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +4.roles in {FUNCTION}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 160.` |
| 221 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = default<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⏎<br>Confidence: 0.999. Support: 462.` |
| 222 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {default}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {SCOPE}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.945. Support: 6849.` |
| 223 | `  •••start_col ≥ 19<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -3.diff_col ≤ 7<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.roles in {ARGUMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 423.` |
| 224 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -4.roles in {EXPRESSION}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 673.` |
| 225 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 2<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 1024.` |
| 226 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.roles in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {FILE}<br>⇒ y = '<br>Confidence: 0.998. Support: 271.` |
| 227 | `  •••start_col ≥ 7<br>	∧ •••start_line ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 701.` |
| 228 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ +1.length ≤ 69<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 24910.` |
| 229 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles not in {BLOCK}<br>	∧ +1.length ≤ 2<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 1299.` |
| 230 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ -2.length ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = ⏎⏎<br>Confidence: 0.947. Support: 779.` |
| 231 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.length = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 683.` |
| 232 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {BINARY}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION, FUNCTION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 401.` |
| 233 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 746.` |
| 234 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≥ 13<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 219.` |
| 235 | `  •••start_col ≥ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 28176.` |
| 236 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 846.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.555084745762711, "max_conf": 0.9997075796127319, "max_support": 32863, "min_conf": 0.9203233122825623, "min_support": 142, "num_rules": 236}}
```
</details>
