# Model report for file:///tmp/top-repos-quality-repos-7uf1o08w/railroad-diagrams.git HEAD d0399118a2fc3cd753ad88af2f457c44511678ad

### Dump

```json
{'created_at': '2021-08-30 08:38:37',
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
 'size': '16.0 kB',
 'tags': [],
 'uuid': '298611a7-ede4-4ea4-97e6-889b40e73207',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-7uf1o08w/railroad-diagrams.git d0399118a2fc3cd753ad88af2f457c44511678ad

# javascript
14 rules, avg.len. 6.4
## train
PPCR: 0.804494
### report
macro
{'f1-score': 0.6138818825106984,
 'precision': 0.6364680320862616,
 'recall': 0.5978597854545108,
 'support': 9917}
micro
{'f1-score': 0.9407078753655339,
 'precision': 0.9407078753655339,
 'recall': 0.9407078753655339,
 'support': 9917}
weighted
{'f1-score': 0.9386600484668203,
 'precision': 0.9378399175388517,
 'recall': 0.9407078753655339,
 'support': 9917}
### report_full
macro
{'f1-score': 0.5582556526359831,
 'precision': 0.6364680320862616,
 'recall': 0.5028703010909007,
 'support': 12327}
micro
{'f1-score': 0.8387879877719835,
 'precision': 0.9407078753655339,
 'recall': 0.7567940293664314,
 'support': 12327}
weighted
{'f1-score': 0.8169476582188918,
 'precision': 0.8895421289092212,
 'recall': 0.7567940293664314,
 'support': 12327}
## test
PPCR: 0.805056
### report
macro
{'f1-score': 0.621756345321988,
 'precision': 0.6390071439258438,
 'recall': 0.608157384063599,
 'support': 9936}
micro
{'f1-score': 0.9323671497584541,
 'precision': 0.9323671497584541,
 'recall': 0.9323671497584541,
 'support': 9936}
weighted
{'f1-score': 0.9306230261386802,
 'precision': 0.9297642556975657,
 'recall': 0.9323671497584541,
 'support': 9936}
### report_full
macro
{'f1-score': 0.5591203458195537,
 'precision': 0.6390071439258438,
 'recall': 0.5029550369443793,
 'support': 12342}
micro
{'f1-score': 0.8316725020199299,
 'precision': 0.9323671497584541,
 'recall': 0.7506076810889645,
 'support': 12342}
weighted
{'f1-score': 0.8086720975694448,
 'precision': 0.8789680418861976,
 'recall': 0.7506076810889645,
 'support': 12342}
```

## javascript
### Summary
7 rules, avg.len. 7.4

| | |
|-|-|
|Min support|96|
|Max support|3794|
|Min confidence|0.9228187799453735|
|Max confidence|0.9947916865348816|

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
| 1 | `  -1.roles not in {LITERAL}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ +2.internal_type not in {NumericLiteral}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 710.` |
| 2 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.945. Support: 100.` |
| 3 | `  -1.reserved not in {;}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 269.` |
| 4 | `  -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 149.` |
| 5 | `  -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 96.` |
| 6 | `  -1.diff_col ≤ 11<br>	∧ -1.reserved not in {), ,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {EXPRESSION} and not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 3794.` |
| 7 | `  -1.diff_col ≤ 11<br>	∧ -1.reserved not in {), ,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION, STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {EXPRESSION, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 777.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.428571428571429, "max_conf": 0.9947916865348816, "max_support": 3794, "min_conf": 0.9228187799453735, "min_support": 96, "num_rules": 7}}
```
</details>
