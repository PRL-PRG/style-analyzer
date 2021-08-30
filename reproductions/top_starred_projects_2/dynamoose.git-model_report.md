# Model report for file:///tmp/top-repos-quality-repos-hp7_ylut/dynamoose.git HEAD 1c933163051248846b1723678c82dffa74a9cb6a

### Dump

```json
{'created_at': '2021-08-30 03:33:52',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.6 kB',
 'tags': [],
 'uuid': 'a6ff3674-6080-4cf7-8297-d561a385c18e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-hp7_ylut/dynamoose.git 1c933163051248846b1723678c82dffa74a9cb6a

# javascript
33 rules, avg.len. 5.7
## train
PPCR: 0.989763
### report
macro
{'f1-score': 0.9463827078358796,
 'precision': 0.9687171526528645,
 'recall': 0.926891632627319,
 'support': 97559}
micro
{'f1-score': 0.980832111850265,
 'precision': 0.980832111850265,
 'recall': 0.980832111850265,
 'support': 97559}
weighted
{'f1-score': 0.9806292002541441,
 'precision': 0.980815263742716,
 'recall': 0.980832111850265,
 'support': 97559}
### report_full
macro
{'f1-score': 0.9314620690845076,
 'precision': 0.9687171526528645,
 'recall': 0.8992805625164688,
 'support': 98568}
micro
{'f1-score': 0.975786097783579,
 'precision': 0.980832111850265,
 'recall': 0.9707917376836296,
 'support': 98568}
weighted
{'f1-score': 0.9750482550988908,
 'precision': 0.9803057993666897,
 'recall': 0.9707917376836296,
 'support': 98568}
## test
PPCR: 0.984805
### report
macro
{'f1-score': 0.8089437752360319,
 'precision': 0.8904263971176262,
 'recall': 0.7787856238102766,
 'support': 9527}
micro
{'f1-score': 0.9487771596515168,
 'precision': 0.9487771596515168,
 'recall': 0.9487771596515168,
 'support': 9527}
weighted
{'f1-score': 0.9468476989804002,
 'precision': 0.9490995875000677,
 'recall': 0.9487771596515168,
 'support': 9527}
### report_full
macro
{'f1-score': 0.7930513911308926,
 'precision': 0.8904263971176262,
 'recall': 0.7468818281994803,
 'support': 9674}
micro
{'f1-score': 0.9415134628404771,
 'precision': 0.9487771596515168,
 'recall': 0.934360140583006,
 'support': 9674}
weighted
{'f1-score': 0.9385056393847323,
 'precision': 0.9469828720992287,
 'recall': 0.934360140583006,
 'support': 9674}
```

## javascript
### Summary
26 rules, avg.len. 5.4

| | |
|-|-|
|Min support|93|
|Max support|32919|
|Min confidence|0.9225146174430847|
|Max confidence|0.9999514222145081|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 1.000. Support: 10297.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -4.label in {"}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.reserved = :<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.999. Support: 513.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.label in {"}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.reserved not in {:}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 2265.` |
| 4 | `  •••start_col ≥ 42<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -4.label not in {"}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.923. Support: 1026.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_col ≥ 26<br>	∧ -2.label in {<newline>}<br>	∧ -4.label not in {"}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.941. Support: 93.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<newline>}<br>	∧ -4.label not in {"}<br>	∧ -4.reserved = .<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 7<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.939. Support: 156.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<newline>}<br>	∧ -4.label not in {"}<br>	∧ -4.reserved not in {.}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.991. Support: 9909.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 3925.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2224.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.999. Support: 895.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +4.reserved = ,<br>⇒ y = ⏎⏎<br>Confidence: 0.997. Support: 583.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +4.reserved not in {,}<br>⇒ y = ⏎<br>Confidence: 0.926. Support: 1604.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1354.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type not in {VariableDeclarator}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 1309.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.954. Support: 953.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, {}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.941. Support: 520.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, {}<br>	∧ -3.roles in {LEFT}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 480.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 366.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = async<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 279.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = await<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 234.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = new<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 154.` |
| 22 | `  •••start_col ≥ 37<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, new, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 1723.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 99.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, return, {}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 486.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +2.length ≤ 8<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 506.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, return, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 32919.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.423076923076923, "max_conf": 0.9999514222145081, "max_support": 32919, "min_conf": 0.9225146174430847, "min_support": 93, "num_rules": 26}}
```
</details>
