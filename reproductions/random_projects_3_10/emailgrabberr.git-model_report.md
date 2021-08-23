# Model report for file:///tmp/top-repos-quality-repos-sj6wdxje/emailgrabberr.git HEAD 14951eccfbfed778e94226fa1c8809cd65b30967

### Dump

```json
{'created_at': '2021-08-22 03:09:10',
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
 'size': '17.2 kB',
 'tags': [],
 'uuid': 'aee563a3-26e2-4627-b841-bb722cbf0c4c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-sj6wdxje/emailgrabberr.git 14951eccfbfed778e94226fa1c8809cd65b30967

# javascript
18 rules, avg.len. 5.3
## train
PPCR: 0.913840
### report
macro
{'f1-score': 0.6900322562645982,
 'precision': 0.7036074449032675,
 'recall': 0.6796729303322284,
 'support': 13873}
micro
{'f1-score': 0.9362791032941685,
 'precision': 0.9362791032941685,
 'recall': 0.9362791032941685,
 'support': 13873}
weighted
{'f1-score': 0.9294922493671129,
 'precision': 0.9238142585357266,
 'recall': 0.9362791032941685,
 'support': 13873}
### report_full
macro
{'f1-score': 0.6556386329173853,
 'precision': 0.7036074449032675,
 'recall': 0.6175095947336917,
 'support': 15181}
micro
{'f1-score': 0.8941281751221862,
 'precision': 0.9362791032941685,
 'recall': 0.8556089849153548,
 'support': 15181}
weighted
{'f1-score': 0.8796283527411206,
 'precision': 0.9071594324008337,
 'recall': 0.8556089849153548,
 'support': 15181}
## test
PPCR: 0.864307
### report
macro
{'f1-score': 0.6540651473255432,
 'precision': 0.6557298170020086,
 'recall': 0.6530586530801789,
 'support': 2051}
micro
{'f1-score': 0.9336908824963432,
 'precision': 0.9336908824963432,
 'recall': 0.9336908824963432,
 'support': 2051}
weighted
{'f1-score': 0.9292335589182956,
 'precision': 0.9252906959918125,
 'recall': 0.9336908824963432,
 'support': 2051}
### report_full
macro
{'f1-score': 0.5765714866789109,
 'precision': 0.6557298170020086,
 'recall': 0.5377333585721519,
 'support': 2373}
micro
{'f1-score': 0.8657323688969258,
 'precision': 0.9336908824963432,
 'recall': 0.8069953645174884,
 'support': 2373}
weighted
{'f1-score': 0.8422704452884483,
 'precision': 0.924423341810847,
 'recall': 0.8069953645174884,
 'support': 2373}
```

## javascript
### Summary
10 rules, avg.len. 5.9

| | |
|-|-|
|Min support|90|
|Max support|5323|
|Min confidence|0.9407894611358643|
|Max confidence|0.9972677826881409|

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
                     'min_samples_split': 190,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles in {STRING}<br>	∧ ^1.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.982. Support: 656.` |
| 2 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.989. Support: 222.` |
| 3 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 421.` |
| 4 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.980. Support: 279.` |
| 5 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 380.` |
| 6 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 274.` |
| 7 | `  -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 183.` |
| 8 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 160.` |
| 9 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 90.` |
| 10 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, if, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 5323.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.9, "max_conf": 0.9972677826881409, "max_support": 5323, "min_conf": 0.9407894611358643, "min_support": 90, "num_rules": 10}}
```
</details>
